use crate::local_datetime;
use chrono::{Local, NaiveDateTime, TimeZone};
use sqlx::FromRow;
use stock_metrics_kernel::model::market_kind::{
    value::{MarketCode, MarketName},
    MarketKind, NewMarketKind,
};

#[derive(FromRow, Debug)]
pub struct MarketKindTable {
    pub id: String,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl MarketKindTable {
    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

impl TryFrom<MarketKindTable> for MarketKind {
    type Error = anyhow::Error;
    fn try_from(mt: MarketKindTable) -> Result<Self, Self::Error> {
        Ok(MarketKind::new(
            mt.id.try_into()?,
            MarketCode(mt.code),
            MarketName(mt.name),
            local_datetime!(mt.created_at),
            local_datetime!(mt.updated_at),
        ))
    }
}

impl TryFrom<NewMarketKind> for MarketKindTable {
    type Error = anyhow::Error;
    fn try_from(mt: NewMarketKind) -> Result<Self, Self::Error> {
        Ok(MarketKindTable {
            id: mt.id.value.to_string(),
            code: mt.code.0,
            name: mt.name.0,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        })
    }
}
