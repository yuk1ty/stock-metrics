use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, response::IntoResponse};
use tracing::error;

use crate::{
    context::validate::ValidatedRequest, model::market_kind::JsonCreateMarketKind, module::Modules,
};

#[tracing::instrument]
pub async fn create_market_kind(
    ValidatedRequest(source): ValidatedRequest<JsonCreateMarketKind>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules
        .market_kind_use_case()
        .register_market_kind(source.into())
        .await;
    res.map(|id| (StatusCode::OK, id)).map_err(|err| {
        error!("Unexpected error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
