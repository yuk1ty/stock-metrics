use async_trait::async_trait;
use sqlx::query_as;
use stock_metrics_kernel::{
    model::stock::{Stock, StockId},
    repository::stock::StockRepository,
};

use crate::{model::stock::StockTable, persistence::mysql::Db};

pub struct StockRepositoryImpl<'a> {
    pool: &'a Db,
}

impl<'a> StockRepositoryImpl<'a> {
    pub fn new(pool: &'static Db) -> StockRepositoryImpl<'a> {
        StockRepositoryImpl { pool }
    }
}

#[async_trait]
impl<'a> StockRepository for StockRepositoryImpl<'a> {
    async fn find(&self, id: StockId) -> anyhow::Result<Stock> {
        let pool = self.pool.0.clone();
        let stock_table = query_as::<_, StockTable>("select * from stock where id = ?")
            .bind(id.0)
            .fetch_one(&*pool)
            .await?;
        let stock = StockTable::try_into(stock_table)?;
        Ok(stock)
    }

    async fn insert(&self, source: Stock) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let stock_table: StockTable = Stock::try_into(source)?;
        let _ = sqlx::query(
            "insert into stock (id, name, ticker_symbol, market_kind) values (?, ?, ?, ?)",
        )
        .bind(stock_table.id)
        .bind(stock_table.name)
        .bind(stock_table.ticker_symbol)
        .bind(stock_table.market_kind)
        .execute(&*pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use chrono::Local;
    use stock_metrics_kernel::model::stock::{
        market_kind::MarketKind, ticker_symbol::TickerSymbol, Stock, StockId,
    };
    use stock_metrics_kernel::repository::stock::StockRepository;

    use crate::persistence::mysql::Db;

    use super::StockRepositoryImpl;

    #[tokio::test]
    async fn test_insert_stock() {
        let db = Db::new().await;
        let repository = StockRepositoryImpl { pool: &db };
        let _ = repository
            .insert(Stock::new(
                StockId("bcd".to_string()),
                "NIKKEI225".to_string(),
                TickerSymbol("NIKKEI225".to_string()),
                MarketKind::try_from("TSE".to_string()).unwrap(),
                Local::now(),
                Local::now(),
            ))
            .await
            .unwrap();
        let found = repository.find(StockId("bcd".to_string())).await.unwrap();
        assert_eq!(found.id.0, "bcd".to_string());
    }
}
