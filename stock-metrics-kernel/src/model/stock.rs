use chrono::{DateTime, Local};
use derive_new::new;

use self::ticker_symbol::TickerSymbol;

use crate::model::market_kind::MarketKind;

use super::Id;

pub mod ticker_symbol;

#[derive(new, Debug)]
pub struct Stock {
    pub id: Id<Stock>,
    pub name: String,
    pub ticker_symbol: TickerSymbol,
    pub market_kind: Id<MarketKind>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(new, Debug)]
pub struct NewStock {
    pub id: Id<Stock>,
    pub name: String,
    pub ticker_symbol: TickerSymbol,
    pub market_kind: Id<MarketKind>,
}
