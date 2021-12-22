use derive_new::new;
use stock_metrics_kernel::repository::{market_kind::MarketKindRepository, stock::StockRepository};

use crate::model::stock_view::StockView;

#[derive(new)]
// TODO あとで型パラメータを変える
pub struct StockViewUseCase<StockRepo, MarketKindRepo>
where
    StockRepo: StockRepository,
    MarketKindRepo: MarketKindRepository,
{
    repository: StockRepo,
    market_kind_repository: MarketKindRepo,
}

impl<StockRepo, MarketKindRepo> StockViewUseCase<StockRepo, MarketKindRepo>
where
    StockRepo: StockRepository,
    MarketKindRepo: MarketKindRepository,
{
    pub async fn show_specific_stock(&self, id: String) -> anyhow::Result<Option<StockView>> {
        let stock = self.repository.find(id.try_into()?).await?;
        match stock {
            Some(stock) => {
                let market_kind = self.market_kind_repository.find(&stock.market_kind).await?;
                Ok(market_kind.map(|market_kind| StockView::new(stock, market_kind)))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod test {
    use stock_metrics_adapter::{
        persistence::mysql::Db,
        repository::{stock::StockRepositoryImpl, DatabaseRepositoryImpl},
    };
    use tokio::sync::OnceCell;

    use super::StockViewUseCase;

    static DB: OnceCell<Db> = OnceCell::const_new();

    pub fn db() -> &'static Db {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let db = rt.block_on(async { DB.get_or_init(Db::new).await });
        db
    }

    #[tokio::test]
    async fn create_usecase() {
        // let db = Db::new().await;
        let repository = StockRepositoryImpl::new(db());
        let market_kind_repository = DatabaseRepositoryImpl::new(db());
        // TODO mock
        let usecase = StockViewUseCase::new(repository, market_kind_repository);
        let _ = usecase
            .show_specific_stock("bcd".to_string())
            .await
            .unwrap();
    }
}
