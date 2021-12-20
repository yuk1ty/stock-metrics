use std::sync::Arc;

use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub struct Db(pub Arc<Pool<MySql>>);

impl Db {
    pub async fn new() -> Db {
        let pool = MySqlPoolOptions::new()
            .max_connections(8)
            .connect("mysql://root:password@localhost/test_stock")
            .await
            .unwrap_or_else(|_| {
                panic!("Cannot connect to the database. Please check your configuration.")
            });
        Db(Arc::new(pool))
    }
}
