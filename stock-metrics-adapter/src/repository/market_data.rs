use std::collections::HashMap;

use async_trait::async_trait;
use aws_sdk_dynamodb::model::{AttributeValue, PutRequest, WriteRequest};
use stock_metrics_kernel::{
    model::market_data::NewMarketData, repository::market_data::MarketDataRepository,
};

use super::DynamoDBRepositoryImpl;

#[async_trait]
impl MarketDataRepository for DynamoDBRepositoryImpl<NewMarketData> {
    async fn insert(&self, source: Vec<NewMarketData>) -> anyhow::Result<()> {
        Ok(())
    }
}