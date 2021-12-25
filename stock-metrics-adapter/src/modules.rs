use stock_metrics_kernel::{
    model::{market_kind::MarketKind, stock::Stock},
    repository::{market_kind::MarketKindRepository, stock::StockRepository},
};

use crate::{persistence::mysql::Db, repository::DatabaseRepositoryImpl};

pub struct RepositoriesModule {
    stock_repository: DatabaseRepositoryImpl<Stock>,
    market_kind_repository: DatabaseRepositoryImpl<MarketKind>,
}

pub trait RepositoriesModuleExt {
    type StockRepo: StockRepository;
    type MarketKindRepo: MarketKindRepository;
    fn stock_repository(&self) -> &Self::StockRepo;
    fn market_kind_repository(&self) -> &Self::MarketKindRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type StockRepo = DatabaseRepositoryImpl<Stock>;
    type MarketKindRepo = DatabaseRepositoryImpl<MarketKind>;

    fn stock_repository(&self) -> &Self::StockRepo {
        &self.stock_repository
    }

    fn market_kind_repository(&self) -> &Self::MarketKindRepo {
        &self.market_kind_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let stock_repository = DatabaseRepositoryImpl::new(db.clone());
        let market_kind_repository = DatabaseRepositoryImpl::new(db.clone());
        Self {
            stock_repository,
            market_kind_repository,
        }
    }
}
