use serde::Deserialize;
use stock_metrics_app::model::market_kind::CreateMarketKind;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct JsonCreateMarketKind {
    #[validate(length(min = 1, max = 36))]
    code: String,
    #[validate(length(min = 1, max = 255))]
    name: String,
}

impl From<JsonCreateMarketKind> for CreateMarketKind {
    fn from(s: JsonCreateMarketKind) -> Self {
        CreateMarketKind {
            code: s.code,
            name: s.name,
        }
    }
}
