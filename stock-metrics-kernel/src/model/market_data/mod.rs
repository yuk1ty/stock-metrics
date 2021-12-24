use chrono::{DateTime, Local};
use derive_new::new;

use super::{stock::Stock, Id};

#[derive(new)]
pub struct NewMarketData {
    stock_id: Id<Stock>,
    as_of: DateTime<Local>,
    start_price: f32,
    end_price: f32,
    high_price: f32,
}
