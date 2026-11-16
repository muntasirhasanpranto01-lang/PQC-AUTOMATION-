use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use crypto_discovery::{build_discovery_result, DiscoveryRequest, DiscoveryResult};
use guardian_core::GuardianContext;
use migration_architect::{propose_migration, MigrationPlan, MigrationRequest};
use pqc_intelligence::{build_intelligence_report, IntelligenceContext};
use quantum_risk::{assess_risk, RiskAssessment, RiskRequest};
use serde::Serialize;
use sqlx::{migrate::Migrator, PgPool};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;

mod repository;
mod service;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[derive(Clone)]
struct AppState {
    service: Arc<service::GuardianService>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug)]
enum ApiError {
    BadRequest(String),
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".into()),
        };
        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

async fn health(State(state): State<AppState>) -> Result<&'static str, ApiError> {
    state.service.health().await.map_err(|_| ApiError::Internal)?;
    Ok("ok")
}

async fn discovery_handler(
    State(state): State<AppState>,
    Json(request): Json<DiscoveryRequest>,
) -> Result<Json<DiscoveryResult>, ApiError> {
    let result = build_discovery_result(request)
        .map_err(|error| ApiError::BadRequest(error.to_string()))?;
    state
        .service
        .record_discovery(&result)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(Json(result))
}

async fn risk_handler(
    Json(request): Json<RiskRequest>,
) -> Result<Json<RiskAssessment>, ApiError> {
    assess_risk(request)
        .map(Json)
        .map_err(|error| ApiError::BadRequest(error.to_string()))
}

async fn intelligence_handler(
    Json(context): Json<GuardianContext>,
) -> Result<Json<IntelligenceContext>, ApiError> {
    build_intelligence_report(context)
        .map(Json)
        .map_err(|error| ApiError::BadRequest(error.to_string()))
}

async fn migration_handler(
    Json(request): Json<MigrationRequest>,
) -> Result<Json<MigrationPlan>, ApiError> {
    propose_migration(request)
        .map(Json)
        .map_err(|error| ApiError::BadRequest(error.to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL")?;
    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let pool = PgPool::connect(&database_url).await?;
    MIGRATOR.run(&pool).await?;

    let service = Arc::new(service::GuardianService::new(pool));
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/discovery/inventory", post(discovery_handler))
        .route("/api/v1/risk/assess", post(risk_handler))
        .route("/api/v1/intelligence/algorithms", post(intelligence_handler))
        .route("/api/v1/migrations/plan", post(migration_handler))
        .with_state(AppState { service })
        .layer(TraceLayer::new_for_http());

    let address: SocketAddr = bind_address.parse()?;
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(%address, "PQC Guardian API listening");
    axum::serve(listener, app).await?;
    Ok(())
}
