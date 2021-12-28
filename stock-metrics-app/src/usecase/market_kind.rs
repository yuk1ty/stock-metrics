use std::sync::Arc;

use derive_new::new;
use stock_metrics_adapter::modules::RepositoriesModuleExt;
use stock_metrics_kernel::{
    model::market_kind::MarketKind, repository::market_kind::MarketKindRepository,
};

use crate::model::market_kind::CreateMarketKind;

#[derive(new)]
pub struct MarketKindUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> MarketKindUseCase<R> {
    pub async fn find(&self, id: String) -> anyhow::Result<Option<MarketKind>> {
        self.repositories
            .market_kind_repository()
            .find(&id.try_into()?)
            .await
            .map(|kind| kind.map(|m| m.into()))
    }

    pub async fn register_market_kind(&self, source: CreateMarketKind) -> anyhow::Result<String> {
        self.repositories
            .market_kind_repository()
            .insert(source.try_into()?)
            .await
            .map(|id| id.value.to_string())
    }

    pub async fn delete_market_kind(&self, id: String) -> anyhow::Result<()> {
        self.repositories
            .market_kind_repository()
            .delete(&id.try_into()?)
            .await
    }
}
