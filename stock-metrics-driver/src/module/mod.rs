use futures::executor::block_on;
use stock_metrics_adapter::{
    persistence::mysql::Db,
    repository::{stock::StockRepositoryImpl, DatabaseRepositoryImpl},
};
use stock_metrics_app::usecase::{
    market_kind::MarketKindUseCase, stock::StockUseCase, stock_view::StockViewUseCase,
};
use stock_metrics_kernel::repository::{market_kind::MarketKindRepository, stock::StockRepository};
use tokio::sync::OnceCell;

// TODO module の作りがよくなくて、ここのフィールドにもたせるようにする必要がある？？
#[derive(Debug)]
pub struct Modules {}

static DB: OnceCell<Db> = OnceCell::const_new();

impl Modules {
    pub(super) fn db(&self) -> &'static Db {
        // This can't capitalize on tokio's Runtime#block_on because
        // we can't launch another runtime on the existing tokio's runtime.
        // Alternatively we take advantage of futures crate's runtime.
        let db = block_on(DB.get_or_init(Db::new));
        db
    }

    fn stock_repository(&self) -> impl StockRepository {
        let repository = StockRepositoryImpl::new(self.db());
        repository
    }

    fn market_kind_repository(&self) -> impl MarketKindRepository {
        let repository = DatabaseRepositoryImpl::new(self.db());
        repository
    }

    pub fn stock_view_use_case(&self) -> StockViewUseCase<impl StockRepository> {
        let usecase = StockViewUseCase::new(self.stock_repository());
        usecase
    }

    pub fn stock_use_case(&self) -> StockUseCase<impl StockRepository> {
        let usecase = StockUseCase::new(self.stock_repository());
        usecase
    }

    pub fn market_kind_use_case(&self) -> MarketKindUseCase<impl MarketKindRepository> {
        let usecase = MarketKindUseCase::new(self.market_kind_repository());
        usecase
    }
}
