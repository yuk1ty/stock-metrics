use chrono::{DateTime, Local};
use derive_new::new;

use self::{market_kind::MarketKind, ticker_symbol::TickerSymbol};

use super::Id;

pub mod market_kind;
pub mod ticker_symbol;

#[derive(new)]
pub struct Stock {
    pub id: Id<Stock>,
    pub name: String,
    pub ticker_symbol: TickerSymbol,
    pub market_kind: Id<MarketKind>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(new)]
pub struct NewStock {
    pub id: Id<Stock>,
    pub name: String,
    pub ticker_symbol: TickerSymbol,
    pub market_kind: Id<MarketKind>,
}
