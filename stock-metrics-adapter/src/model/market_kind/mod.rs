use chrono::{DateTime, Local};
use sqlx::FromRow;
use stock_metrics_kernel::model::market_kind::{
    value::{MarketCode, MarketName},
    MarketKind, NewMarketKind,
};

#[derive(FromRow)]
pub struct MarketKindTable {
    pub id: String,
    pub code: u32,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
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
            mt.created_at,
            mt.updated_at,
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
            created_at: Local::now(),
            updated_at: Local::now(),
        })
    }
}
