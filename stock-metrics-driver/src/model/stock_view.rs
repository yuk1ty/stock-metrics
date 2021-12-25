use serde::Serialize;
use stock_metrics_app::model::stock_view::StockView;

#[derive(Serialize)]
pub struct JsonStockView {
    id: String,
    name: String,
    ticker_symbol: String,
    market_kind: String,
}

impl From<StockView> for JsonStockView {
    fn from(s: StockView) -> Self {
        JsonStockView {
            id: s.id,
            name: s.name,
            ticker_symbol: s.ticker_symbol,
            market_kind: s.market_kind_name,
        }
    }
}
