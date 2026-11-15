use serde::Serialize;
use utoipa::ToSchema;

/// Wire format for `GET /health/live` and `/health/ready`.
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: &'static str,
}

/// Wire format for `GET /api/v1/system/info`.
#[derive(Debug, Serialize, ToSchema)]
pub struct SystemInfoResponse {
    pub service: String,
    pub version: String,
    pub environment: String,
    pub status: &'static str,
}
