use derive_new::new;
use stock_metrics_kernel::repository::stock::StockRepository;

use crate::model::stock::CreateStock;

#[derive(new)]
pub struct StockUseCase<R: StockRepository> {
    repository: R,
}

impl<R: StockRepository> StockUseCase<R> {
    pub async fn register_stock(&self, source: CreateStock) -> anyhow::Result<()> {
        self.repository.insert(source.try_into()?).await
    }
}
