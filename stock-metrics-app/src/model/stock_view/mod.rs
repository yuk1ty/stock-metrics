use stock_metrics_kernel::model::{market_kind::MarketKind, stock::Stock};

pub struct StockView {
    pub id: String,
    pub name: String,
    pub ticker_symbol: String,
    pub market_kind_name: String,
}

impl StockView {
    pub fn new(stock: Stock, market_kind: MarketKind) -> Self {
        Self {
            id: stock.id.value.to_string(),
            name: stock.name,
            ticker_symbol: stock.ticker_symbol.0,
            market_kind_name: market_kind.name.0,
        }
    }
}
