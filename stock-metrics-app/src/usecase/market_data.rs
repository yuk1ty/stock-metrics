use std::sync::Arc;

use derive_new::new;
use stock_metrics_adapter::modules::RepositoriesModuleExt;

use crate::model::market_data::MarketData;

#[derive(new)]
pub struct MarketDataUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> MarketDataUseCase<R> {
    pub fn register_market_data(&self, source: Vec<MarketData>) -> anyhow::Result<()> {
        Ok(())
    }
}
