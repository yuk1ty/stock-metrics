use serde::Deserialize;
use stock_metrics_app::model::stock::CreateStock;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct JsonCreateStock {
    #[validate(length(min = 1, max = 255))]
    name: String,
    #[validate(length(min = 1, max = 255))]
    ticker_symbol: String,
    market_kind: String,
}

impl From<JsonCreateStock> for CreateStock {
    fn from(s: JsonCreateStock) -> Self {
        CreateStock {
            name: s.name,
            ticker_symbol: s.ticker_symbol,
            market_kind: s.market_kind,
        }
    }
}
