use anyhow::anyhow;
use chrono::{Date, Local, NaiveDate, TimeZone};

#[derive(Debug)]
pub struct MarketData {
    pub stock_id: String,
    pub as_of: Date<Local>,
    pub start_price: f32,
    pub end_price: f32,
    pub high_price: f32,
    pub low_price: f32,
}

impl MarketData {
    pub fn new(
        stock_id: String,
        as_of: String,
        start_price: f32,
        end_price: f32,
        high_price: f32,
        low_price: f32,
    ) -> anyhow::Result<MarketData> {
        Ok(Self {
            stock_id,
            as_of: Local
                .from_local_date(&NaiveDate::parse_from_str(as_of.as_str(), "%Y/%m/%d")?)
                .single()
                .ok_or(anyhow!("Cannot parse date: {}", as_of))?,
            start_price,
            end_price,
            high_price,
            low_price,
        })
    }
}
