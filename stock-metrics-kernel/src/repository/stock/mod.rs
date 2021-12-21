use async_trait::async_trait;

use crate::model::stock::{Stock, StockId};

#[async_trait]
pub trait StockRepository {
    async fn find(&self, id: StockId) -> anyhow::Result<Option<Stock>>;
    async fn insert(&self, source: Stock) -> anyhow::Result<()>;
}
