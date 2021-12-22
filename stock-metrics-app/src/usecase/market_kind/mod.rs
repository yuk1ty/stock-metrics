use derive_new::new;
use stock_metrics_kernel::{
    model::market_kind::MarketKind, repository::market_kind::MarketKindRepository,
};

use crate::model::market_kind::CreateMarketKind;

#[derive(new)]
pub struct MarketKindUseCase<R: MarketKindRepository> {
    repository: R,
}

impl<R: MarketKindRepository> MarketKindUseCase<R> {
    pub async fn find(&self, id: String) -> anyhow::Result<Option<MarketKind>> {
        self.repository
            .find(&id.try_into()?)
            .await
            .map(|kind| kind.map(|m| m.into()))
    }

    pub async fn register_market_kind(&self, source: CreateMarketKind) -> anyhow::Result<String> {
        self.repository
            .insert(source.try_into()?)
            .await
            .map(|id| id.value.to_string())
    }
}
