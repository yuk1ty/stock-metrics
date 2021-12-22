use std::sync::Arc;

use derive_new::new;
use stock_metrics_adapter::modules::RepositoriesModuleExt;
use stock_metrics_kernel::repository::stock::StockRepository;

use crate::model::stock::CreateStock;

#[derive(new)]
pub struct StockUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> StockUseCase<R> {
    pub async fn register_stock(&self, source: CreateStock) -> anyhow::Result<()> {
        self.repositories
            .stock_repository()
            .insert(source.try_into()?)
            .await
    }
}
