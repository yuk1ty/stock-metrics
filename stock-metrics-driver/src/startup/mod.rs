use crate::{
    module::Modules,
    routes::stock_view::{create_stock, stock_view},
};
use axum::{handler::get, AddExtensionLayer, Router};
use derive_new::new;
use std::{net::SocketAddr, sync::Arc};

#[derive(new)]
pub struct Server {
    modules: Arc<Modules>,
}

impl Server {
    pub async fn startup(self) {
        let app = Router::new()
            .route("/stocks", get(stock_view).post(create_stock))
            .layer(AddExtensionLayer::new(self.modules));

        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap_or_else(|_| panic!("Server cannot launch!"))
    }
}
