use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    context::validate::ValidatedRequest,
    model::{stock::CreateStock, stock_view::JsonStockView},
    module::Modules,
};

pub async fn stock_view(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.stock_view_use_case().show_specific_stock(id).await;
    match res {
        Ok(sv) => sv
            .map(|sv| {
                let json: JsonStockView = sv.into();
                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| StatusCode::NOT_FOUND),
        Err(err) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_stock(
    ValidatedRequest(source): ValidatedRequest<CreateStock>,
    Extension(module): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("{:?}", source);
    Ok(StatusCode::OK)
}
