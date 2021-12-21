use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct CreateStock {
    #[validate(length(min = 1, max = 255))]
    name: String,
    #[validate(length(min = 1, max = 255))]
    ticker_symbol: String,
    market_kind: String,
}
