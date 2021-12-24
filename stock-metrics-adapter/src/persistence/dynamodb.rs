use std::sync::Arc;

use aws_config::load_from_env;
use aws_sdk_dynamodb::config::Builder;
use aws_sdk_dynamodb::{Client, Endpoint};
use http::Uri;

pub struct DynamoDB {
    pub(crate) client: Arc<Client>,
}

pub async fn init_client() -> Client {
    let config = load_from_env().await;
    let config = Builder::from(&config)
        .endpoint_resolver(Endpoint::immutable(Uri::from_static(
            "http://localhost:8000",
        )))
        .build();
    let dynamodb = Client::from_conf(config);
    dynamodb
}

impl DynamoDB {
    pub fn new(client: Client) -> DynamoDB {
        DynamoDB {
            client: Arc::new(client),
        }
    }

    pub async fn list_tables(&self) -> anyhow::Result<Option<Vec<String>>> {
        let res = self.client.list_tables().send().await?;
        Ok(res.table_names)
    }
}

#[cfg(test)]
mod test {
    use super::{init_client, DynamoDB};

    #[tokio::test]
    #[ignore]
    async fn test_list_table() {
        let client = init_client().await;
        let dynamodb = DynamoDB::new(client);
        let _ = dynamodb.list_tables().await;
    }
}
