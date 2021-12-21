use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, response::IntoResponse};

use crate::module::Modules;

pub async fn hc() -> Result<impl IntoResponse, StatusCode> {
    tracing::debug!("Access health check endpoint from user!");
    Ok(StatusCode::NO_CONTENT)
}

pub async fn hc_db(
    Extension(module): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO connection test with querying to database through some repositories
    let pool = module.db().0.clone();
    let experiment = pool.try_acquire().ok_or(StatusCode::INTERNAL_SERVER_ERROR);
    experiment.map(|pool| {
        pool.detach();
        StatusCode::OK
    })
}
