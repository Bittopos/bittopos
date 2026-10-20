#!/usr/bin/env python3
"""Small HTTP/SQLite persistence adapter for the Bittopos browser prototype.

This is an API boundary experiment, not the semantic core and not a production
multi-user service. See docs/api-backed-frontend.md before exposing it to the internet.
"""
from __future__ import annotations

import json
import os
import re
import sqlite3
import tomllib
from datetime import date, datetime, time
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote, urlparse

ROOT = Path(__file__).resolve().parent
WEB_ROOT = ROOT.parent / "web"
DB_PATH = Path(os.environ.get("BITTOPOS_DB_PATH", ROOT / "data" / "bittopos.sqlite3"))
API_TOKEN = os.environ.get("BITTOPOS_API_TOKEN", "")
CORS_ORIGIN = os.environ.get("BITTOPOS_CORS_ORIGIN", "")
MAX_BODY = 5 * 1024 * 1024
DEFAULT_WORKSPACE = {"content": {"instances": []}}


def json_default(value):
    if isinstance(value, (datetime, date, time)):
        return value.isoformat()
    raise TypeError(f"Value is not JSON serializable: {type(value).__name__}")


def dumps(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"), default=json_default)


def connect():
    DB_PATH.parent.mkdir(parents=True, exist_ok=True)
    db = sqlite3.connect(DB_PATH, timeout=10)
    db.row_factory = sqlite3.Row
    db.execute("PRAGMA foreign_keys = ON")
    db.execute("PRAGMA journal_mode = WAL")
    return db


def init_db():
    with connect() as db:
        db.execute("""
            CREATE TABLE IF NOT EXISTS resources (
                id TEXT PRIMARY KEY,
                document_json TEXT NOT NULL,
                source_toml TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        """)
        db.execute("""
            CREATE TABLE IF NOT EXISTS workspaces (
                id TEXT PRIMARY KEY,
                document_json TEXT NOT NULL,
                revision INTEGER NOT NULL DEFAULT 0,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        """)
        db.execute(
            "INSERT OR IGNORE INTO workspaces(id, document_json, revision) VALUES (?, ?, 0)",
            ("default", dumps(DEFAULT_WORKSPACE)),
        )


def validate_resource(value):
    if not isinstance(value, dict) or not isinstance(value.get("id"), str) or not value["id"].strip():
        raise ValueError("El recurso debe conservar un id de texto.")
    info = value.get("info")
    if not isinstance(info, dict) or not isinstance(info.get("title"), str) or not info["title"].strip():
        raise ValueError("El recurso debe contener info.title.")
    return value


def validate_workspace(value):
    if not isinstance(value, dict) or not isinstance(value.get("content"), dict):
        raise ValueError("La estructura debe contener content.instances.")
    if not isinstance(value["content"].get("instances"), list):
        raise ValueError("content.instances debe ser una lista.")
    seen = set()

    def walk(instances):
        for instance in instances:
            if not isinstance(instance, dict):
                raise ValueError("Cada instancia debe ser un objeto JSON.")
            instance_id = instance.get("id")
            source = instance.get("source")
            if not isinstance(instance_id, str) or not instance_id:
                raise ValueError("Cada instancia debe conservar su id.")
            if instance_id in seen:
                raise ValueError(f"ID de instancia duplicado: {instance_id}")
            seen.add(instance_id)
            if not isinstance(source, dict) or not isinstance(source.get("id"), str):
                raise ValueError(f"La instancia {instance_id} debe conservar source.id.")
            content = instance.get("content")
            if content is not None:
                if not isinstance(content, dict) or not isinstance(content.get("instances"), list):
                    raise ValueError(f"content.instances inválido en {instance_id}.")
                walk(content["instances"])

    walk(value["content"]["instances"])
    return value


def get_state():
    with connect() as db:
        resources = [
            json.loads(row["document_json"])
            for row in db.execute("SELECT document_json FROM resources ORDER BY created_at, id")
        ]
        row = db.execute(
            "SELECT document_json, revision FROM workspaces WHERE id = 'default'"
        ).fetchone()
        return {"resources": resources, "workspace": json.loads(row["document_json"]), "revision": row["revision"]}


def import_toml(source):
    try:
        document = tomllib.loads(source)
    except tomllib.TOMLDecodeError as exc:
        raise ValueError(f"TOML inválido: {exc}") from exc
    resource = validate_resource(document)
    with connect() as db:
        db.execute(
            """INSERT INTO resources(id, document_json, source_toml)
               VALUES (?, ?, ?)
               ON CONFLICT(id) DO UPDATE SET document_json=excluded.document_json,
                 source_toml=excluded.source_toml, updated_at=CURRENT_TIMESTAMP""",
            (resource["id"], dumps(resource), source),
        )
    return resource


def save_resource(resource_id, document):
    resource = validate_resource(document)
    if resource["id"] != resource_id:
        raise ValueError("El id de la URL debe coincidir con el id del recurso.")
    with connect() as db:
        cursor = db.execute(
            """UPDATE resources SET document_json=?, source_toml=NULL, updated_at=CURRENT_TIMESTAMP
               WHERE id=?""",
            (dumps(resource), resource_id),
        )
        if cursor.rowcount == 0:
            raise KeyError(resource_id)
    return resource


def create_resource(document):
    resource = validate_resource(document)
    with connect() as db:
        try:
            db.execute(
                "INSERT INTO resources(id, document_json) VALUES (?, ?)",
                (resource["id"], dumps(resource)),
            )
        except sqlite3.IntegrityError as exc:
            raise FileExistsError(resource["id"]) from exc
    return resource


def save_workspace(document, expected_revision):
    workspace = validate_workspace(document)
    with connect() as db:
        db.execute("BEGIN IMMEDIATE")
        row = db.execute(
            "SELECT revision FROM workspaces WHERE id='default'"
        ).fetchone()
        current = row["revision"]
        if expected_revision is not None and expected_revision != current:
            db.rollback()
            raise RuntimeError(f"REVISION_CONFLICT:{current}")
        next_revision = current + 1
        db.execute(
            """UPDATE workspaces SET document_json=?, revision=?, updated_at=CURRENT_TIMESTAMP
               WHERE id='default'""",
            (dumps(workspace), next_revision),
        )
        db.commit()
    return {"workspace": workspace, "revision": next_revision}


def resource_toml(resource_id):
    with connect() as db:
        row = db.execute(
            "SELECT source_toml, document_json FROM resources WHERE id=?",
            (resource_id,),
        ).fetchone()
    if row is None:
        raise KeyError(resource_id)
    if row["source_toml"] is not None:
        return row["source_toml"]
    # Lossless TOML serialization of arbitrary resource documents is deliberately
    # not guessed here. Edited/imported-from-JSON documents remain available as JSON.
    raise ValueError("Este recurso no conserva su TOML original. Exporta el documento JSON para evitar pérdida de campos.")


class Handler(SimpleHTTPRequestHandler):
    server_version = "BittoposAPI/0.1"

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(WEB_ROOT), **kwargs)

    def end_headers(self):
        if CORS_ORIGIN:
            self.send_header("Access-Control-Allow-Origin", CORS_ORIGIN)
            self.send_header("Vary", "Origin")
            self.send_header("Access-Control-Allow-Headers", "Authorization, Content-Type, If-Match")
            self.send_header("Access-Control-Allow-Methods", "GET, POST, PUT, OPTIONS")
        super().end_headers()

    def do_OPTIONS(self):
        self.send_response(204)
        self.end_headers()

    def send_json(self, status, value):
        body = dumps(value).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def send_text(self, status, value, content_type="text/plain; charset=utf-8"):
        body = value.encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def read_json(self):
        try:
            length = int(self.headers.get("Content-Length", "0"))
        except ValueError:
            raise ValueError("Content-Length inválido.")
        if length < 1 or length > MAX_BODY:
            raise ValueError("El cuerpo debe tener entre 1 byte y 5 MiB.")
        try:
            return json.loads(self.rfile.read(length))
        except (json.JSONDecodeError, UnicodeDecodeError) as exc:
            raise ValueError("El cuerpo debe ser JSON válido UTF-8.") from exc

    def authorized(self):
        if not API_TOKEN:
            return True
        return self.headers.get("Authorization", "") == f"Bearer {API_TOKEN}"

    def do_GET(self):
        parsed = urlparse(self.path)
        if not parsed.path.startswith("/api/v1/"):
            if parsed.path == "/":
                self.path = "/index.html"
            return super().do_GET()
        if parsed.path == "/api/v1/health":
            return self.send_json(200, {"ok": True, "service": "bittopos-api", "version": "v1"})
        if not self.authorized():
            return self.send_json(401, {"error": "Se requiere un token válido."})
        try:
            if parsed.path == "/api/v1/state":
                return self.send_json(200, get_state())
            match = re.fullmatch(r"/api/v1/resources/([^/]+)/toml", parsed.path)
            if match:
                return self.send_text(200, resource_toml(unquote(match.group(1))), "application/toml; charset=utf-8")
            return self.send_json(404, {"error": "Ruta API no encontrada."})
        except KeyError:
            return self.send_json(404, {"error": "Recurso no encontrado."})
        except ValueError as exc:
            return self.send_json(409, {"error": str(exc)})

    def do_POST(self):
        parsed = urlparse(self.path)
        if not parsed.path.startswith("/api/v1/"):
            return self.send_json(404, {"error": "Ruta API no encontrada."})
        if not self.authorized():
            return self.send_json(401, {"error": "Se requiere un token válido."})
        try:
            if parsed.path == "/api/v1/resources/import-toml":
                payload = self.read_json()
                if not isinstance(payload, dict) or not isinstance(payload.get("content"), str):
                    raise ValueError("Se esperaba {content: string} con el TOML original.")
                resource = import_toml(payload["content"])
                return self.send_json(201, {"resource": resource})
            if parsed.path == "/api/v1/resources":
                resource = create_resource(self.read_json())
                return self.send_json(201, {"resource": resource})
            return self.send_json(404, {"error": "Ruta API no encontrada."})
        except ValueError as exc:
            return self.send_json(400, {"error": str(exc)})
        except FileExistsError as exc:
            return self.send_json(409, {"error": f"Ya existe el recurso {exc}."})

    def do_PUT(self):
        parsed = urlparse(self.path)
        if not parsed.path.startswith("/api/v1/"):
            return self.send_json(404, {"error": "Ruta API no encontrada."})
        if not self.authorized():
            return self.send_json(401, {"error": "Se requiere un token válido."})
        try:
            if parsed.path == "/api/v1/workspace":
                payload = self.read_json()
                if not isinstance(payload, dict) or "workspace" not in payload:
                    raise ValueError("Se esperaba {workspace, expectedRevision}.")
                expected = payload.get("expectedRevision")
                if expected is not None and (not isinstance(expected, int) or expected < 0):
                    raise ValueError("expectedRevision debe ser un entero no negativo.")
                return self.send_json(200, save_workspace(payload["workspace"], expected))
            match = re.fullmatch(r"/api/v1/resources/([^/]+)", parsed.path)
            if match:
                resource = save_resource(unquote(match.group(1)), self.read_json())
                return self.send_json(200, {"resource": resource})
            return self.send_json(404, {"error": "Ruta API no encontrada."})
        except KeyError:
            return self.send_json(404, {"error": "Recurso no encontrado."})
        except RuntimeError as exc:
            message = str(exc)
            if message.startswith("REVISION_CONFLICT:"):
                current = int(message.split(":", 1)[1])
                return self.send_json(409, {"error": "La estructura cambió en otra sesión.", "revision": current})
            return self.send_json(400, {"error": message})
        except ValueError as exc:
            return self.send_json(400, {"error": str(exc)})

    def log_message(self, fmt, *args):
        if os.environ.get("BITTOPOS_HTTP_LOG", "1") != "0":
            super().log_message(fmt, *args)


def main():
    init_db()
    host = os.environ.get("BITTOPOS_HOST", "127.0.0.1")
    port = int(os.environ.get("BITTOPOS_PORT", "8787"))
    if host not in {"127.0.0.1", "::1", "localhost"} and not API_TOKEN:
        raise RuntimeError("Refusing to bind to a non-loopback address without BITTOPOS_API_TOKEN. This prototype is not production-hardened.")
    server = ThreadingHTTPServer((host, port), Handler)
    print(f"Bittopos API escuchando en http://{host}:{port} (SQLite: {DB_PATH})")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nCerrando Bittopos API.")
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
