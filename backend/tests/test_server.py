import json
import tempfile
import unittest
from pathlib import Path

from backend import server


class ApiPersistenceTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.previous_path = server.DB_PATH
        server.DB_PATH = Path(self.temp.name) / "test.sqlite3"
        server.init_db()

    def tearDown(self):
        server.DB_PATH = self.previous_path
        self.temp.cleanup()

    def test_state_starts_with_empty_workspace(self):
        state = server.get_state()
        self.assertEqual(state["workspace"], {"content": {"instances": []}})
        self.assertEqual(state["resources"], [])
        self.assertEqual(state["revision"], 0)

    def test_resource_keeps_unknown_fields_and_toml_source(self):
        source = 'id = "r1"\ncustom = { keep = true }\n\n[info]\ntitle = "Proyecto"\ntags = ["x"]\n'
        resource = server.import_toml(source)
        state = server.get_state()
        self.assertEqual(resource["custom"], {"keep": True})
        self.assertEqual(state["resources"][0]["id"], "r1")
        self.assertEqual(server.resource_toml("r1"), source)

    def test_workspace_preserves_instance_fields_and_checks_revision(self):
        workspace = {"content": {"instances": [{"id": "i1", "source": {"id": "r1"}, "custom": {"keep": 7}}]}}
        result = server.save_workspace(workspace, 0)
        self.assertEqual(result["revision"], 1)
        self.assertEqual(server.get_state()["workspace"]["content"]["instances"][0]["custom"], {"keep": 7})
        with self.assertRaisesRegex(RuntimeError, "REVISION_CONFLICT"):
            server.save_workspace(workspace, 0)

    def test_workspace_rejects_duplicate_instance_ids(self):
        workspace = {"content": {"instances": [
            {"id": "same", "source": {"id": "r1"}},
            {"id": "same", "source": {"id": "r2"}},
        ]}}
        with self.assertRaisesRegex(ValueError, "duplicado"):
            server.save_workspace(workspace, 0)

    def test_resource_id_must_match_path(self):
        server.create_resource({"id": "r1", "info": {"title": "Uno"}})
        with self.assertRaisesRegex(ValueError, "coincidir"):
            server.save_resource("other", {"id": "r1", "info": {"title": "Uno"}})


if __name__ == "__main__":
    unittest.main()
