use std::marker::PhantomData;

use crate::persistence::mysql::Db;

pub mod health_check;
pub mod market_kind;
pub mod stock;

pub struct DatabaseRepositoryImpl<'a, T> {
    pool: &'a Db,
    _marker: PhantomData<T>,
}

impl<'a, T> DatabaseRepositoryImpl<'a, T> {
    pub fn new(pool: &'static Db) -> DatabaseRepositoryImpl<'a, T> {
        Self {
            pool,
            _marker: PhantomData,
        }
    }
}
