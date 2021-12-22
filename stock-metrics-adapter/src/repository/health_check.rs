use crate::persistence::dynamodb::DynamoDB;

pub struct HealthCheckRepository<'a> {
    dynamo_db: &'a DynamoDB<'a>,
}

impl<'a> HealthCheckRepository<'a> {
    pub fn new(dynamo_db: &'a DynamoDB<'a>) -> Self {
        Self { dynamo_db }
    }

    pub async fn check_dynamo_db(&self) -> anyhow::Result<()> {
        let _ = self.dynamo_db.list_tables().await?;
        Ok(())
    }
}
