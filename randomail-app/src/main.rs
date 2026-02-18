use std::sync::Arc;

use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::{delete, get},
};
use serde::{Deserialize, Serialize};

use randomail_api::{
    cf_email::{add_email_route, delete_email_route, list_email_routes, update_email_route},
    config::RMConfig,
};

const INDEX_HTML: &str = include_str!("../static/index.html");
const FAVICON: &[u8] = include_bytes!("../static/favicon.ico");

struct AppState {
    config: RMConfig,
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = serde_json::json!({ "error": self.0.to_string() });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn list_aliases(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    let aliases = list_email_routes(&state.config.zone_id, &state.config.token)?;
    let json: Vec<serde_json::Value> = aliases
        .into_iter()
        .map(serde_json::to_value)
        .collect::<Result<_, _>>()?;
    Ok(Json(json))
}

#[derive(Deserialize)]
struct CreateAlias {
    alias: String,
    description: String,
}

async fn create_alias(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAlias>,
) -> Result<StatusCode, AppError> {
    let email_alias = format!("{}@{}", payload.alias, state.config.zone);
    add_email_route(
        &state.config.zone_id,
        payload.description,
        email_alias,
        &state.config.destination_email,
        &state.config.token,
    )?;
    Ok(StatusCode::CREATED)
}

async fn remove_alias(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    delete_email_route(&state.config.zone_id, &id, &state.config.token)?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
struct UpdateAlias {
    enabled: bool,
}

async fn toggle_alias(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAlias>,
) -> Result<StatusCode, AppError> {
    update_email_route(
        &state.config.zone_id,
        &id,
        &state.config.token,
        payload.enabled,
    )?;
    Ok(StatusCode::OK)
}

#[derive(Serialize)]
struct ConfigResponse {
    account_id: String,
    destination_email: String,
    zone: String,
    version: &'static str,
}

async fn get_config(State(state): State<Arc<AppState>>) -> Json<ConfigResponse> {
    Json(ConfigResponse {
        account_id: state.config.account_id.clone(),
        destination_email: state.config.destination_email.clone(),
        zone: state.config.zone.clone(),
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn favicon() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "image/x-icon")], FAVICON)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = RMConfig::load()?;
    let state = Arc::new(AppState { config });

    let app = Router::new()
        .route("/", get(index))
        .route("/favicon.ico", get(favicon))
        .route("/aliases", get(list_aliases).post(create_alias))
        .route("/aliases/{id}", delete(remove_alias).put(toggle_alias))
        .route("/config", get(get_config))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on http://localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
