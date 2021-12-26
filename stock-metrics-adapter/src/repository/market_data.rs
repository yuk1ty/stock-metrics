use std::collections::HashMap;

use async_trait::async_trait;
use aws_sdk_dynamodb::model::{PutRequest, WriteRequest};
use stock_metrics_kernel::{
    model::market_data::NewMarketData, repository::market_data::MarketDataRepository,
};

use super::DynamoDBRepositoryImpl;

#[async_trait]
impl MarketDataRepository for DynamoDBRepositoryImpl<NewMarketData> {
    async fn insert(&self, source: Vec<NewMarketData>) -> anyhow::Result<()> {
        let market_data: Vec<WriteRequest> = source
            .into_iter()
            .map(|m| {
                WriteRequest::builder()
                    .set_put_request(Some(PutRequest::builder().build()))
                    .build()
            })
            .collect();
        let mut attr_map = HashMap::new();
        attr_map.insert("stock_market_data".to_string(), market_data);
        self.dynamoDB
            .client
            .batch_write_item()
            .set_request_items(Some(attr_map));
        Ok(())
    }
}
