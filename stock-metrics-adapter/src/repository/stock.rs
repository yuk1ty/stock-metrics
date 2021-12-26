use async_trait::async_trait;
use sqlx::query_as;
use stock_metrics_kernel::{
    model::{
        stock::{NewStock, Stock},
        Id,
    },
    repository::stock::StockRepository,
};

use crate::model::stock::StockTable;

use super::DatabaseRepositoryImpl;

#[async_trait]
impl StockRepository for DatabaseRepositoryImpl<Stock> {
    async fn find(&self, id: &Id<Stock>) -> anyhow::Result<Option<Stock>> {
        let pool = self.pool.0.clone();
        let stock_table = query_as::<_, StockTable>("select * from stock where id = ?")
            .bind(id.value.to_string())
            .fetch_one(&*pool)
            .await
            .ok();
        match stock_table {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn insert(&self, source: NewStock) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let stock_table: StockTable = source.try_into()?;
        let _ = sqlx::query(
            "insert into stock (id, name, ticker_symbol, market_kind, created_at, updated_at) values (?, ?, ?, ?, ?, ?)",
        )
        .bind(stock_table.id)
        .bind(stock_table.name)
        .bind(stock_table.ticker_symbol)
        .bind(stock_table.market_kind)
        .bind(stock_table.created_at)
        .bind(stock_table.updated_at)
        .execute(&*pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use stock_metrics_kernel::model::stock::ticker_symbol::TickerSymbol;
    use stock_metrics_kernel::model::stock::NewStock;
    use stock_metrics_kernel::model::Id;
    use stock_metrics_kernel::repository::stock::StockRepository;
    use ulid::Ulid;

    use crate::persistence::mysql::Db;

    use super::DatabaseRepositoryImpl;

    // TODO later fix
    #[ignore]
    #[tokio::test]
    async fn test_insert_stock() {
        let db = Db::new().await;
        let repository = DatabaseRepositoryImpl::new(db);
        let id = Ulid::new();
        let _ = repository
            .insert(NewStock::new(
                Id::new(id),
                "NIKKEI225".to_string(),
                TickerSymbol("NIKKEI225".to_string()),
                Id::new(id),
            ))
            .await
            .unwrap();
        let found = repository.find(&Id::new(id)).await.unwrap().unwrap();
        assert_eq!(found.id.value, id);
    }
}
