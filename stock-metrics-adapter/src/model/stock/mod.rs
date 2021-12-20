use chrono::{DateTime, Local};
use sqlx::FromRow;
use stock_metrics_kernel::model::stock::{
    market_kind::MarketKind, ticker_symbol::TickerSymbol, Stock, StockId,
};

#[derive(FromRow)]
pub struct StockTable {
    pub id: String,
    pub name: String,
    pub ticker_symbol: String,
    pub market_kind: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl TryFrom<StockTable> for Stock {
    type Error = anyhow::Error;
    fn try_from(st: StockTable) -> Result<Self, Self::Error> {
        Ok(Stock::new(
            StockId(st.id),
            st.name,
            TickerSymbol(st.ticker_symbol),
            MarketKind::try_from(st.market_kind)?,
            st.created_at,
            st.updated_at,
        ))
    }
}

impl TryFrom<Stock> for StockTable {
    type Error = anyhow::Error;
    fn try_from(s: Stock) -> Result<Self, Self::Error> {
        Ok(StockTable {
            id: s.id.0,
            name: s.name,
            ticker_symbol: s.ticker_symbol.0,
            market_kind: s.market_kind.to_string(),
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
    }
}
