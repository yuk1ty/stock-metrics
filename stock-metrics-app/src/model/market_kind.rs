use derive_new::new;
use stock_metrics_kernel::model::{
    market_kind::{
        value::{MarketCode, MarketName},
        NewMarketKind,
    },
    Id,
};

#[derive(new)]
pub struct CreateMarketKind {
    pub code: String,
    pub name: String,
}

impl TryFrom<CreateMarketKind> for NewMarketKind {
    type Error = anyhow::Error;
    fn try_from(c: CreateMarketKind) -> anyhow::Result<Self> {
        let market_kind_id = Id::gen();
        Ok(NewMarketKind::new(
            market_kind_id,
            MarketCode(c.code),
            MarketName(c.name),
        ))
    }
}
