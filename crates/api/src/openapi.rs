//! OpenAPI 3.1 document generation.
//!
//! Baseline document is generated from `utoipa`. Path annotations are added
//! as real endpoints are implemented; `iaop-api-contracts` remains the
//! canonical contract source of truth.

use utoipa::OpenApi;

use crate::dto::{HealthResponse, SystemInfoResponse};
use crate::routes::{live, ready, system_info};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Eligibility and Authorization API",
        version = "0.1.0",
        description = "IAOP Eligibility and Authorization service API. Contracts are governed by iaop-api-contracts."
    ),
    servers((url = "/")),
    paths(live, ready, system_info),
    components(schemas(HealthResponse, SystemInfoResponse))
)]
pub struct ApiDoc;
