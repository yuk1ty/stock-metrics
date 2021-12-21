use derive_new::new;
use stock_metrics_kernel::repository::market_kind::MarketKindRepository;

use crate::model::market_kind::CreateMarketKind;

#[derive(new)]
pub struct MarketKindUseCase<R: MarketKindRepository> {
    repository: R,
}

impl<R: MarketKindRepository> MarketKindUseCase<R> {
    pub async fn register_market_kind(&self, source: CreateMarketKind) -> anyhow::Result<String> {
        self.repository
            .insert(source.try_into()?)
            .await
            .map(|id| id.value.to_string())
    }
}
