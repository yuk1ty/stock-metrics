use chrono::{DateTime, Local};
use derive_new::new;

use self::value::{MarketCode, MarketName};

use super::Id;

pub mod value;

#[derive(new, Debug)]
pub struct MarketKind {
    pub id: Id<MarketKind>,
    pub code: MarketCode,
    pub name: MarketName,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(new)]
pub struct NewMarketKind {
    pub id: Id<MarketKind>,
    pub code: MarketCode,
    pub name: MarketName,
}
