use derive_new::new;
use stock_metrics_kernel::{model::stock::StockId, repository::stock::StockRepository};

use crate::model::stock_view::StockView;

#[derive(new)]
// TODO あとで型パラメータを変える
pub struct StockViewUseCase<R: StockRepository> {
    repository: R,
}

impl<R: StockRepository> StockViewUseCase<R> {
    pub async fn show_specific_stock(&self, id: String) -> anyhow::Result<StockView> {
        self.repository
            .find(StockId(id))
            .await
            .map(|stock| stock.into())
    }
}

#[cfg(test)]
mod test {
    use stock_metrics_adapter::{persistence::mysql::Db, repository::stock::StockRepositoryImpl};
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
        // TODO mock
        let usecase = StockViewUseCase::new(repository);
        let _ = usecase
            .show_specific_stock("bcd".to_string())
            .await
            .unwrap();
    }
}
