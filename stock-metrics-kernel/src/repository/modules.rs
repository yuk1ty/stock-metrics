use super::stock::StockRepository;

pub trait RepositoryModule {
    type StockRepository: StockRepository;

    fn stock_repository(&self) -> Self::StockRepository;
}
