use async_trait::async_trait;

use crate::model::{
    stock::{NewStock, Stock},
    Id,
};

#[async_trait]
pub trait StockRepository {
    async fn find(&self, id: &Id<Stock>) -> anyhow::Result<Option<Stock>>;
    async fn insert(&self, source: NewStock) -> anyhow::Result<()>;
}
