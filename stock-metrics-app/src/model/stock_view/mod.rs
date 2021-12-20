use stock_metrics_kernel::model::stock::Stock;

pub struct StockView {
    pub id: String,
    pub name: String,
    pub ticker_symbol: String,
    pub market_kind: String,
}

impl From<Stock> for StockView {
    fn from(s: Stock) -> Self {
        StockView {
            id: s.id.0,
            name: s.name,
            ticker_symbol: s.ticker_symbol.0,
            market_kind: s.market_kind.to_string(),
        }
    }
}
