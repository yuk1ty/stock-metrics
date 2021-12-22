use std::sync::Arc;

use crate::persistence::dynamodb::DynamoDB;

pub struct HealthCheckRepository {
    dynamo_db: Arc<DynamoDB>,
}

impl HealthCheckRepository {
    pub fn new(dynamo_db: DynamoDB) -> Self {
        Self {
            dynamo_db: Arc::new(dynamo_db),
        }
    }

    pub async fn check_dynamo_db(&self) -> anyhow::Result<()> {
        let _ = self.dynamo_db.list_tables().await?;
        Ok(())
    }
}
