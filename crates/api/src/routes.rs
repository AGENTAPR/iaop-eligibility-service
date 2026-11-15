//! HTTP route handlers.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use iaop_eligibility_service_application::service_info::GetServiceInfoQuery;
use iaop_eligibility_service_infrastructure::persistence;
use utoipa::OpenApi;

use crate::dto::{HealthResponse, SystemInfoResponse};
use crate::middleware;
use crate::openapi::ApiDoc;
use crate::state::AppState;

/// Builds the service router with all baseline endpoints.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health/live", get(live))
        .route("/health/ready", get(ready))
        .route("/api/v1/system/info", get(system_info))
        .route("/api/docs/openapi.json", get(openapi_json))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::request_context,
        ))
        .with_state(state)
}

/// Liveness: process is up. Must NOT depend on external systems.
#[utoipa::path(get, path = "/health/live", tag = "ops", operation_id = "getHealthLive", summary = "Liveness probe", responses((status = 200, description = "Process is alive", body = HealthResponse)))]
pub(crate) async fn live() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthResponse { status: "ok" }))
}

/// Readiness: dependencies required to serve traffic.
/// - No DB configured: ready (DB intentionally disabled).
/// - DB configured: ready only if a query succeeds.
#[utoipa::path(get, path = "/health/ready", tag = "ops", operation_id = "getHealthReady", summary = "Readiness probe", description = "Ready only when required dependencies (PostgreSQL when configured) are reachable.", responses((status = 200, description = "Ready", body = HealthResponse),(status = 503, description = "Degraded — dependency unavailable", body = HealthResponse)))]
pub(crate) async fn ready(State(state): State<AppState>) -> impl IntoResponse {
    match &state.db {
        Some(pool) => {
            if persistence::ping(pool).await {
                (StatusCode::OK, Json(HealthResponse { status: "ready" }))
            } else {
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(HealthResponse { status: "degraded" }),
                )
            }
        }
        None => (StatusCode::OK, Json(HealthResponse { status: "ready" })),
    }
}

#[utoipa::path(get, path = "/api/v1/system/info", tag = "ops", operation_id = "getSystemInfo", summary = "Service metadata", responses((status = 200, description = "Service info", body = SystemInfoResponse)))]
pub(crate) async fn system_info(State(state): State<AppState>) -> impl IntoResponse {
    let info = GetServiceInfoQuery {
        service_name: state.config.service_name.clone(),
        service_version: state.config.service_version.clone(),
        environment: state.config.environment.clone(),
    }
    .execute();
    Json(SystemInfoResponse {
        service: info.service,
        version: info.version,
        environment: info.environment,
        status: info.status,
    })
}

async fn openapi_json() -> impl IntoResponse {
    Json(ApiDoc::openapi())
}
