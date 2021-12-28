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
impl MarketKindRepository for DatabaseRepositoryImpl<MarketKind> {
    async fn find(&self, id: &Id<MarketKind>) -> anyhow::Result<Option<MarketKind>> {
        let pool = self.pool.0.clone();
        let market_kind = sqlx::query_as!(
            MarketKindTable,
            "select * from market_kind where id = ?",
            id.value.to_string()
        )
        .fetch_one(&*pool)
        .await
        .ok();
        match market_kind {
            Some(mk) => {
                let mk = mk.try_into()?;
                Ok(Some(mk))
            }
            None => Ok(None),
        }
    }

    async fn insert(&self, source: NewMarketKind) -> anyhow::Result<Id<MarketKind>> {
        let pool = self.pool.0.clone();
        let market_kind_table: MarketKindTable = source.try_into()?;
        let _ = sqlx::query!(
		    "insert into market_kind (id, code, name, created_at, updated_at) values (?, ?, ?, ?, ?)",
            market_kind_table.id(),
            market_kind_table.code,
            market_kind_table.name,
            market_kind_table.created_at,
            market_kind_table.updated_at,
        )
        .execute(&*pool)
        .await?;
        Ok(market_kind_table.id.try_into()?)
    }

    async fn delete(&self, id: &Id<MarketKind>) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let _ = sqlx::query!("delete from market_kind where id = ?", id.value.to_string())
            .execute(&*pool)
            .await?;
        Ok(())
    }
}
