use std::marker::PhantomData;

use crate::persistence::mysql::Db;

pub mod health_check;
pub mod market_kind;
pub mod stock;

pub struct DatabaseRepositoryImpl<T> {
    pool: Db,
    _marker: PhantomData<T>,
}

impl<T> DatabaseRepositoryImpl<T> {
    pub fn new(pool: Db) -> DatabaseRepositoryImpl<T> {
        Self {
            pool,
            _marker: PhantomData,
        }
    }
}
