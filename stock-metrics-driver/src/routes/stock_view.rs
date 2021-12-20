use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{model::stock_view::JsonStockView, module::Modules};

pub async fn stock_view(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .stock_view_use_case()
        .show_specific_stock(id)
        .await
        .map(|s| {
            let response_body: JsonStockView = s.into();
            (StatusCode::OK, Json(response_body))
        })
        // TODO error handling
        .map_err(|_| StatusCode::NOT_FOUND)
}
