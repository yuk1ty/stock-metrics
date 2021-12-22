use stock_metrics_adapter::repository::health_check::HealthCheckRepository;

pub struct HealthCheckUseCase<'a> {
    repository: &'a HealthCheckRepository<'a>,
}

impl<'a> HealthCheckUseCase<'a> {
    pub fn new(repository: &'a HealthCheckRepository<'a>) -> Self {
        Self { repository }
    }

    pub async fn check_dynamo_db(&self) -> anyhow::Result<()> {
        self.repository.check_dynamo_db().await
    }
}
