use async_trait::async_trait;

use crate::model::{
    market_kind::{MarketKind, NewMarketKind},
    Id,
};

#[async_trait]
pub trait MarketKindRepository {
    async fn find(&self, id: &Id<MarketKind>) -> anyhow::Result<Option<MarketKind>>;
    async fn insert(&self, source: NewMarketKind) -> anyhow::Result<Id<MarketKind>>;
    async fn delete(&self, id: &Id<MarketKind>) -> anyhow::Result<()>;
}
