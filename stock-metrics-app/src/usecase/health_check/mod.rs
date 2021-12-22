use std::sync::Arc;

use stock_metrics_adapter::repository::health_check::HealthCheckRepository;

pub struct HealthCheckUseCase {
    repository: Arc<HealthCheckRepository>,
}

impl HealthCheckUseCase {
    pub fn new(repository: HealthCheckRepository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }

    pub async fn check_dynamo_db(&self) -> anyhow::Result<()> {
        self.repository.check_dynamo_db().await
    }
}
