use std::sync::Arc;

use stock_metrics_driver::{module::Modules, startup::Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = Server::new(Arc::new(Modules {}));
    let _ = server.startup().await;
    Ok(())
}
