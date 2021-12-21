use crate::model::market_kind::MarketKindTable;

use super::DatabaseRepositoryImpl;
use async_trait::async_trait;
use stock_metrics_kernel::{
    model::{
        market_kind::{MarketKind, NewMarketKind},
        Id,
    },
    repository::market_kind::MarketKindRepository,
};

#[async_trait]
impl<'a> MarketKindRepository for DatabaseRepositoryImpl<'a, MarketKind> {
    async fn find(&self, id: Id<MarketKind>) -> anyhow::Result<Option<MarketKind>> {
        unimplemented!()
    }

    async fn insert(&self, source: NewMarketKind) -> anyhow::Result<Id<MarketKind>> {
        let pool = self.pool.0.clone();
        let market_kind_table: MarketKindTable = source.try_into()?;
        let _ = sqlx::query(
			"insert into market_kind (id, code, name, created_at, updated_at) values (?, ?, ?, ?, ?)"
		)
		.bind(market_kind_table.id())
		.bind(market_kind_table.code)
		.bind(market_kind_table.name)
		.bind(market_kind_table.created_at)
		.bind(market_kind_table.updated_at)
		.execute(&*pool)
		.await?;
        Ok(market_kind_table.id.try_into()?)
    }
}
