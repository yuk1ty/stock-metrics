use derive_new::new;
use stock_metrics_kernel::model::{
    stock::{ticker_symbol::TickerSymbol, NewStock},
    Id,
};

#[derive(new)]
pub struct CreateStock {
    pub name: String,
    pub ticker_symbol: String,
    pub market_kind: String,
}

impl TryFrom<CreateStock> for NewStock {
    type Error = anyhow::Error;

    fn try_from(c: CreateStock) -> anyhow::Result<Self> {
        let stock_id = Id::gen();
        Ok(NewStock::new(
            stock_id,
            c.name,
            TickerSymbol(c.ticker_symbol),
            c.market_kind.try_into()?,
        ))
    }
}
