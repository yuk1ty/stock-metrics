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
        let market_data: Vec<WriteRequest> = source
            .into_iter()
            .map(|m| {
                WriteRequest::builder()
                    .set_put_request(Some(
                        PutRequest::builder()
                            .set_item(Some(
                                [
                                    ("as_of".to_string(), AttributeValue::S(m.as_of.to_string())),
                                    (
                                        "stock_id".to_string(),
                                        AttributeValue::S(m.stock_id.value.to_string()),
                                    ),
                                    (
                                        "start_price".to_string(),
                                        AttributeValue::N(m.start_price.to_string()),
                                    ),
                                    (
                                        "end_price".to_string(),
                                        AttributeValue::N(m.end_price.to_string()),
                                    ),
                                    (
                                        "high_price".to_string(),
                                        AttributeValue::N(m.high_price.to_string()),
                                    ),
                                    (
                                        "low_price".to_string(),
                                        AttributeValue::N(m.low_price.to_string()),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ))
                            .build(),
                    ))
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
