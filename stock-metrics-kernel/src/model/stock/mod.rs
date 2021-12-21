use chrono::{DateTime, Local};
use derive_new::new;
use uuid::Uuid;

use self::{market_kind::MarketKind, ticker_symbol::TickerSymbol};

pub mod market_kind;
pub mod ticker_symbol;

#[derive(new)]
pub struct Stock {
    pub id: StockId,
    pub name: String,
    pub ticker_symbol: TickerSymbol,
    pub market_kind: MarketKind,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(PartialEq)]
pub struct StockId(pub String);

impl StockId {
    pub fn gen() -> StockId {
        let id = Uuid::new_v4().to_string();
        StockId(id)
    }
}

#[derive(new)]
pub struct NewStock {
    pub id: StockId,
    pub name: String,
    pub ticker_symbol: TickerSymbol,
    pub market_kind: MarketKind,
}
