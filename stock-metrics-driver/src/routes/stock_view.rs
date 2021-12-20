use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, response::IntoResponse};

use crate::module::Modules;

pub async fn stock_view(Extension(modules): Extension<Arc<Modules>>) -> impl IntoResponse {
    let stock = modules
        .stock_view_use_case()
        .show_specific_stock("bcd".to_string())
        .await;
    StatusCode::OK
}
