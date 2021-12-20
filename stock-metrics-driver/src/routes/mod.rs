use axum::{handler::get, Router};

use crate::module::Modules;

pub mod stock_view;

pub struct AppRouter {
    modules: Modules,
}

// impl AppRouter {
//     pub fn startup(self) {
//         let router = Router::new().route("/stocks", get(|| self.modules.stock_view_route().view()));
//     }
// }
