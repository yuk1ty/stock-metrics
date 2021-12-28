use crate::{
    module::Modules,
    routes::{
        health::{hc, hc_db, hc_dynamo},
        market_data::upload_market_data,
        market_kind::{create_market_kind, delete_market_kind},
        stock_view::{create_stock, stock_view},
    },
};
use axum::{
    routing::{delete, get, post},
    AddExtensionLayer, Router,
};
use dotenv::dotenv;
use std::{net::SocketAddr, sync::Arc};

pub async fn startup(modules: Arc<Modules>) {
    let hc_router = Router::new()
        .route("/", get(hc))
        .route("/db", get(hc_db))
        .route("/dynamo", get(hc_dynamo));
    let stocks_router = Router::new()
        .route("/", post(create_stock))
        .route("/:id", get(stock_view));
    let market_kind_router = Router::new()
        .route("/", post(create_market_kind))
        .route("/:id", delete(delete_market_kind));
    let market_data_router = Router::new().route("/:stock_id", post(upload_market_data));

    let app = Router::new()
        .nest("/hc", hc_router)
        .nest("/stocks", stocks_router)
        .nest("/market_kind", market_kind_router)
        .nest("/market_data", market_data_router)
        .layer(AddExtensionLayer::new(modules));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"))
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
}
