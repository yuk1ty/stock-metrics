use std::marker::PhantomData;

use derive_new::new;

use crate::persistence::{dynamodb::DynamoDB, mysql::Db};

pub mod health_check;
pub mod market_data;
pub mod market_kind;
pub mod stock;

#[derive(new)]
pub struct DatabaseRepositoryImpl<T> {
    pool: Db,
    _marker: PhantomData<T>,
}

#[derive(new)]
pub struct DynamoDBRepositoryImpl<T> {
    dynamoDB: DynamoDB,
    _marker: PhantomData<T>,
}
