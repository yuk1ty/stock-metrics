use serde::Deserialize;
use stock_metrics_app::model::market_data::MarketData;

#[derive(Deserialize)]
pub struct CsvMarketData {
    as_of: String,
    end_price: String,
    start_price: String,
    high_price: String,
    low_price: String,
    volume: String,
    delta: String,
}

macro_rules! trail_comma {
    ($x:expr) => {
        $x.replace(",", "").parse::<f32>()?
    };
}

macro_rules! formatted_date {
    ($x:expr) => {
        $x.replace("年", "/").replace("月", "/").replace("日", "")
    };
}

impl CsvMarketData {
    pub fn to_market_data(&self, stock_id: impl Into<String>) -> anyhow::Result<MarketData> {
        let start_price = trail_comma!(self.start_price);

        let end_price = trail_comma!(self.end_price);
        let low_price = trail_comma!(self.low_price);
        let high_price = trail_comma!(self.high_price);
        MarketData::new(
            stock_id.into(),
            formatted_date!(self.as_of),
            start_price,
            end_price,
            high_price,
            low_price,
        )
    }
}
