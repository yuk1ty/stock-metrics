use chrono::{DateTime, Local};
use derive_new::new;

#[derive(new)]
pub struct MarketData {
    as_of: DateTime<Local>,
    start_price: f32,
    end_price: f32,
    high_price: f32,
}
