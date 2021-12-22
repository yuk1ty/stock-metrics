use std::sync::Arc;

use stock_metrics_driver::{
    module::Modules,
    startup::{init_app, startup},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_app();

    let modules = Modules::new().await;
    let _ = startup(Arc::new(modules)).await;

    Ok(())
}
