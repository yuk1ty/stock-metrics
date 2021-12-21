use std::sync::Arc;

use stock_metrics_driver::{
    module::Modules,
    startup::{init_app, Server},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_app();

    let server = Server::new(Arc::new(Modules {}));
    let _ = server.startup().await;

    Ok(())
}
