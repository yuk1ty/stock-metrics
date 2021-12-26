use std::sync::Arc;

use derive_new::new;
use stock_metrics_adapter::modules::RepositoriesModuleExt;
use stock_metrics_kernel::repository::{market_kind::MarketKindRepository, stock::StockRepository};

use crate::model::stock_view::StockView;

#[derive(new)]
pub struct StockViewUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> StockViewUseCase<R> {
    pub async fn show_specific_stock(&self, id: String) -> anyhow::Result<Option<StockView>> {
        let stock = self
            .repositories
            .stock_repository()
            .find(&id.try_into()?)
            .await?;
        match stock {
            Some(stock) => {
                let market_kind = self
                    .repositories
                    .market_kind_repository()
                    .find(&stock.market_kind)
                    .await?;
                Ok(market_kind.map(|market_kind| StockView::new(stock, market_kind)))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use stock_metrics_adapter::{modules::RepositoriesModule, persistence::mysql::Db};

    use super::StockViewUseCase;

    #[tokio::test]
    async fn create_usecase() {
        // TODO mock
        let modules = RepositoriesModule::new(Db::new().await);
        let usecase = StockViewUseCase::new(Arc::new(modules));
        let _ = usecase
            .show_specific_stock("bcd".to_string())
            .await
            .unwrap();
    }
}
