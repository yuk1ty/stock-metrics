use derive_new::new;

use self::{market_kind::MarketKind, ticker_symbol::TickerSymbol};

pub mod market_kind;
pub mod ticker_symbol;

#[derive(new)]
pub struct Stock {
    pub id: StockId,
    pub name: String,
    pub ticker_symbol: TickerSymbol,
    pub market_kind: MarketKind,
}

#[derive(PartialEq)]
pub struct StockId(pub String);
