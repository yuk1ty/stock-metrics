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

    pub async fn diagnose_db_conn(&self) -> anyhow::Result<()> {
        self.repository.check_rds_conn().await
    }

    pub async fn diagnose_dynamo_db_conn(&self) -> anyhow::Result<()> {
        self.repository.check_dynamo_db().await
    }
}
