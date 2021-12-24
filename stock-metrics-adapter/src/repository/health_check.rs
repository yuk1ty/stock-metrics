use anyhow::anyhow;
use std::sync::Arc;

use crate::persistence::{dynamodb::DynamoDB, mysql::Db};

pub struct HealthCheckRepository {
    db: Arc<Db>,
    dynamo_db: Arc<DynamoDB>,
}

impl HealthCheckRepository {
    pub fn new(db: Db, dynamo_db: DynamoDB) -> Self {
        Self {
            db: Arc::new(db),
            dynamo_db: Arc::new(dynamo_db),
        }
    }

    pub async fn check_rds_conn(&self) -> anyhow::Result<()> {
        let pool = self.db.0.clone();
        let attempt = pool
            .try_acquire()
            .map(|_| ())
            .ok_or(anyhow!("Failed to connect database."));
        attempt
    }

    pub async fn check_dynamo_db(&self) -> anyhow::Result<()> {
        let _ = self.dynamo_db.list_tables().await?;
        Ok(())
    }
}
