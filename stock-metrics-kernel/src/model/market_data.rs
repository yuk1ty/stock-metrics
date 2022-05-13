#[allow(unused)]
use chrono::{Date, Local};
use derive_new::new;

use super::{stock::Stock, Id};

#[derive(new)]
pub struct NewMarketData {
    pub stock_id: Id<Stock>,
    pub as_of: Date<Local>,
    pub start_price: f32,
    pub end_price: f32,
    pub high_price: f32,
    pub low_price: f32,
}
