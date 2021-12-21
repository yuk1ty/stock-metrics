use crate::{
    module::Modules,
    routes::{
        health::{hc, hc_db},
        market_kind::create_market_kind,
        stock_view::{create_stock, stock_view},
    },
};
use axum::{
    handler::{get, post},
    AddExtensionLayer, Router,
};
use derive_new::new;
use dotenv::dotenv;
use std::{net::SocketAddr, sync::Arc};

#[derive(new)]
pub struct Server {
    modules: Arc<Modules>,
}

impl Server {
    pub async fn startup(self) {
        let hc_router = Router::new().route("/", get(hc)).route("/db", get(hc_db));
        let stocks_router = Router::new()
            .route("/", post(create_stock))
            .route("/:id", get(stock_view));
        let market_kind_router = Router::new().route("/", post(create_market_kind));

        let app = Router::new()
            .nest("/hc", hc_router)
            .nest("/stocks", stocks_router)
            .nest("/market_kind", market_kind_router)
            .layer(AddExtensionLayer::new(self.modules));

        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

        tracing::info!("Server listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap_or_else(|_| panic!("Server cannot launch!"))
    }
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
}
