use std::sync::Arc;

use axum::{
    extract::{ContentLengthLimit, Extension, Multipart, Path},
    http::StatusCode,
    response::IntoResponse,
};
use csv::ReaderBuilder;
use stock_metrics_app::model::market_data::MarketData;
use tracing::error;

use crate::{
    model::market_data::CsvMarketData,
    module::{Modules, ModulesExt},
};

#[tracing::instrument(skip(modules))]
pub async fn upload_market_data(
    Path(stock_id): Path<String>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<Multipart, { 100 * 1024 * 1024 }>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let bytes = field.bytes().await.unwrap();
        let bytes = String::from_utf8(bytes.to_vec()).unwrap();

        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(bytes.as_bytes());

        let mut market_data_list = Vec::new();
        for result in rdr.deserialize() {
            let market_data: CsvMarketData = result.unwrap();
            market_data_list.push(market_data);
        }

        let market_data_list: Vec<MarketData> = market_data_list
            .into_iter()
            .filter_map(|csv| csv.to_market_data(&stock_id).ok())
            .collect();

        let _ = modules
            .market_data_use_case()
            .register_market_data(market_data_list)
            .map_err(|err| {
                error!("Unexpected error: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }
    // TODO queueing the background process and this should be accepted status
    Ok(StatusCode::CREATED)
}
