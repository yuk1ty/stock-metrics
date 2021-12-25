use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, response::IntoResponse};
use tracing::error;

use crate::module::{Modules, ModulesExt};

pub async fn hc() -> impl IntoResponse {
    tracing::debug!("Access health check endpoint from user!");
    StatusCode::NO_CONTENT
}

pub async fn hc_db(
    Extension(module): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    module
        .health_check_use_case()
        .diagnose_db_conn()
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|err| {
            error!("{:?}", err);
            StatusCode::SERVICE_UNAVAILABLE
        })
}

pub async fn hc_dynamo(
    Extension(module): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    module
        .health_check_use_case()
        .diagnose_dynamo_db_conn()
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|err| {
            error!("{:?}", err);
            StatusCode::SERVICE_UNAVAILABLE
        })
}
