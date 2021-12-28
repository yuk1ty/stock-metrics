use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::error;

use crate::{
    context::validate::ValidatedRequest,
    model::market_kind::JsonCreateMarketKind,
    module::{Modules, ModulesExt},
};

#[tracing::instrument(skip(modules))]
pub async fn create_market_kind(
    ValidatedRequest(source): ValidatedRequest<JsonCreateMarketKind>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules
        .market_kind_use_case()
        .register_market_kind(source.into())
        .await;
    res.map(|id| (StatusCode::CREATED, id)).map_err(|err| {
        error!("Unexpected error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip(modules))]
pub async fn delete_market_kind(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .market_kind_use_case()
        .delete_market_kind(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
