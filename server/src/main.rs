use std::{collections::HashMap, env, net::SocketAddr, sync::{Arc, Mutex}, time::{Duration, Instant}};

use axum::{
    extract::{Path, Request, State},
    http::{header, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use bittopos_core::{Resource, ResourceStore, WorkspaceStore};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

const SESSION_COOKIE: &str = "bittopos_session";
const SESSION_TTL: Duration = Duration::from_secs(24 * 60 * 60);

#[derive(Clone)]
struct AppState {
    username: Arc<str>,
    password: Arc<str>,
    secure_cookie: bool,
    resources: Arc<ResourceStore>,
    workspaces: Arc<WorkspaceStore>,
    sessions: Arc<Mutex<HashMap<String, Instant>>>,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct CreateResourceRequest {
    title: String,
    description: Option<String>,
}

fn error(status: StatusCode, message: &'static str) -> Response {
    (status, Json(json!({ "error": message }))).into_response()
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "service": "bittopos-server", "api_version": 1 }))
}

async fn login(State(state): State<AppState>, Json(input): Json<LoginRequest>) -> Response {
    if input.username != state.username.as_ref() || input.password != state.password.as_ref() {
        return error(StatusCode::UNAUTHORIZED, "Credenciales inválidas");
    }

    let token = Uuid::now_v7().to_string();
    let mut sessions = match state.sessions.lock() {
        Ok(sessions) => sessions,
        Err(_) => return error(StatusCode::INTERNAL_SERVER_ERROR, "Sesiones no disponibles"),
    };
    sessions.retain(|_, expiry| *expiry > Instant::now());
    sessions.insert(token.clone(), Instant::now() + SESSION_TTL);
    drop(sessions);

    let same_site = if state.secure_cookie { "None" } else { "Lax" };
    let secure = if state.secure_cookie { "; Secure" } else { "" };
    let cookie = format!(
        "{SESSION_COOKIE}={token}; Path=/; HttpOnly; SameSite={same_site}; Max-Age={}",
        SESSION_TTL.as_secs()
    ) + secure;

    let mut response = Json(json!({ "authenticated": true, "username": state.username.as_ref() }))
        .into_response();
    match HeaderValue::from_str(&cookie) {
        Ok(value) => {
            response.headers_mut().insert(header::SET_COOKIE, value);
            response
        }
        Err(_) => error(StatusCode::INTERNAL_SERVER_ERROR, "No se pudo crear la sesión"),
    }
}

async fn logout(State(state): State<AppState>, request: Request) -> Response {
    if let Some(token) = cookie_value(request.headers(), SESSION_COOKIE) {
        if let Ok(mut sessions) = state.sessions.lock() {
            sessions.remove(&token);
        }
    }

    let cookie = if state.secure_cookie {
        format!("{SESSION_COOKIE}=; Path=/; HttpOnly; SameSite=None; Secure; Max-Age=0")
    } else {
        format!("{SESSION_COOKIE}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0")
    };
    let mut response = Json(json!({ "authenticated": false })).into_response();
    if let Ok(value) = HeaderValue::from_str(&cookie) {
        response.headers_mut().insert(header::SET_COOKIE, value);
    }
    response
}

async fn me(State(state): State<AppState>) -> Json<Value> {
    Json(json!({ "authenticated": true, "username": state.username.as_ref() }))
}

async fn list_resources(State(state): State<AppState>) -> Response {
    match state.resources.list() {
        Ok(resources) => Json(resources).into_response(),
        Err(_) => error(StatusCode::INTERNAL_SERVER_ERROR, "No se pudieron leer los recursos"),
    }
}

async fn get_resource(State(state): State<AppState>, Path(id): Path<Uuid>) -> Response {
    match state.resources.get(id) {
        Ok(resource) => Json(resource).into_response(),
        Err(bittopos_core::StoreError::NotFound(_)) => {
            error(StatusCode::NOT_FOUND, "Recurso no encontrado")
        }
        Err(_) => error(StatusCode::INTERNAL_SERVER_ERROR, "No se pudo leer el recurso"),
    }
}

async fn create_resource(
    State(state): State<AppState>,
    Json(input): Json<CreateResourceRequest>,
) -> Response {
    match state.resources.create(&input.title, input.description.as_deref()) {
        Ok((resource, _warnings)) => (StatusCode::CREATED, Json(resource)).into_response(),
        Err(bittopos_core::StoreError::InvalidName) => {
            error(StatusCode::BAD_REQUEST, "El título no puede estar vacío")
        }
        Err(_) => error(StatusCode::INTERNAL_SERVER_ERROR, "No se pudo crear el recurso"),
    }
}

async fn list_workspaces(State(state): State<AppState>) -> Response {
    match state.workspaces.list() {
        Ok(workspaces) => Json(json!({ "items": workspaces })).into_response(),
        Err(_) => error(StatusCode::INTERNAL_SERVER_ERROR, "No se pudieron leer los espacios"),
    }
}

async fn require_session(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let Some(token) = cookie_value(request.headers(), SESSION_COOKIE) else {
        return error(StatusCode::UNAUTHORIZED, "Autenticación requerida");
    };

    let valid = match state.sessions.lock() {
        Ok(mut sessions) => {
            sessions.retain(|_, expiry| *expiry > Instant::now());
            sessions.get(&token).is_some_and(|expiry| *expiry > Instant::now())
        }
        Err(_) => return error(StatusCode::INTERNAL_SERVER_ERROR, "Sesiones no disponibles"),
    };

    if !valid {
        return error(StatusCode::UNAUTHORIZED, "Sesión inválida o vencida");
    }
    next.run(request).await
}

fn cookie_value(headers: &axum::http::HeaderMap, name: &str) -> Option<String> {
    headers
        .get(header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .filter_map(|part| part.trim().split_once('='))
        .find_map(|(key, value)| (key == name).then(|| value.to_owned()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = env::var("BITTOPOS_PASSWORD")
        .map_err(|_| "Falta BITTOPOS_PASSWORD; el servidor no arrancará sin autenticación")?;
    if password.len() < 12 {
        return Err("BITTOPOS_PASSWORD debe tener al menos 12 caracteres".into());
    }

    let data_dir = env::var("BITTOPOS_DATA_DIR").unwrap_or_else(|_| "./data".to_owned());
    let resources = ResourceStore::new(format!("{data_dir}/resources"))?;
    let workspaces = WorkspaceStore::new(format!("{data_dir}/workspaces"))?;
    let secure_cookie = env::var("SESSION_COOKIE_SECURE")
        .map(|value| value != "false")
        .unwrap_or(true);
    let state = AppState {
        username: Arc::from(env::var("BITTOPOS_USERNAME").unwrap_or_else(|_| "admin".to_owned())),
        password: Arc::from(password),
        secure_cookie,
        resources: Arc::new(resources),
        workspaces: Arc::new(workspaces),
        sessions: Arc::new(Mutex::new(HashMap::new())),
    };

    let origin = env::var("WEB_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".to_owned())
        .parse::<HeaderValue>()?;
    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_credentials(true)
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE]);

    let public = Router::new()
        .route("/health", get(health))
        .route("/api/v1/login", post(login))
        .with_state(state.clone());

    let protected = Router::new()
        .route("/api/v1/logout", post(logout))
        .route("/api/v1/me", get(me))
        .route("/api/v1/resources", get(list_resources).post(create_resource))
        .route("/api/v1/resources/{id}", get(get_resource))
        .route("/api/v1/workspaces", get(list_workspaces))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_session))
        .with_state(state);

    let app = public.merge(protected).layer(cors);
    let address: SocketAddr = env::var("BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_owned())
        .parse()?;
    let listener = TcpListener::bind(address).await?;
    println!("Bittopos server listening on {address}");
    axum::serve(listener, app).await?;
    Ok(())
}
