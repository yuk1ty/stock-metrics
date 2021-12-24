use std::sync::Arc;

use stock_metrics_adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::{
        dynamodb::{init_client, DynamoDB},
        mysql::Db,
    },
    repository::health_check::HealthCheckRepository,
};
use stock_metrics_app::usecase::{
    health_check::HealthCheckUseCase, market_data::MarketDataUseCase,
    market_kind::MarketKindUseCase, stock::StockUseCase, stock_view::StockViewUseCase,
};

pub struct Modules {
    health_check_use_case: HealthCheckUseCase,
    stock_view_use_case: StockViewUseCase<RepositoriesModule>,
    stock_use_case: StockUseCase<RepositoriesModule>,
    market_kind_use_case: MarketKindUseCase<RepositoriesModule>,
    market_data_use_case: MarketDataUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn health_check_use_case(&self) -> &HealthCheckUseCase;
    fn stock_view_use_case(&self) -> &StockViewUseCase<Self::RepositoriesModule>;
    fn stock_use_case(&self) -> &StockUseCase<Self::RepositoriesModule>;
    fn market_kind_use_case(&self) -> &MarketKindUseCase<Self::RepositoriesModule>;
    fn market_data_use_case(&self) -> &MarketDataUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn health_check_use_case(&self) -> &HealthCheckUseCase {
        &self.health_check_use_case
    }

    fn stock_view_use_case(&self) -> &StockViewUseCase<Self::RepositoriesModule> {
        &self.stock_view_use_case
    }

    fn stock_use_case(&self) -> &StockUseCase<Self::RepositoriesModule> {
        &self.stock_use_case
    }

    fn market_kind_use_case(&self) -> &MarketKindUseCase<Self::RepositoriesModule> {
        &self.market_kind_use_case
    }

    fn market_data_use_case(&self) -> &MarketDataUseCase<Self::RepositoriesModule> {
        &self.market_data_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;
        let client = init_client().await;
        let dynamodb = DynamoDB::new(client);

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let health_check_use_case =
            HealthCheckUseCase::new(HealthCheckRepository::new(db, dynamodb));
        let stock_view_use_case = StockViewUseCase::new(repositories_module.clone());
        let stock_use_case = StockUseCase::new(repositories_module.clone());
        let market_kind_use_case = MarketKindUseCase::new(repositories_module.clone());
        let market_data_use_case = MarketDataUseCase::new(repositories_module.clone());

        Self {
            health_check_use_case,
            stock_view_use_case,
            stock_use_case,
            market_kind_use_case,
            market_data_use_case,
        }
    }
}
