use async_trait::async_trait;

use crate::model::market_data::NewMarketData;

#[async_trait]
pub trait MarketDataRepository {
    async fn insert(&self, source: Vec<NewMarketData>) -> anyhow::Result<()>;
}
