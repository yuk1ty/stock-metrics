use anyhow::anyhow;

pub enum MarketKind {
    Tse,
    Ose,
}

impl TryFrom<String> for MarketKind {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "TSE" => Ok(MarketKind::Tse),
            "OSE" => Ok(MarketKind::Ose),
            _ => Err(anyhow!("can't parse MarketKind value")),
        }
    }
}

impl ToString for MarketKind {
    fn to_string(&self) -> String {
        match self {
            MarketKind::Tse => "TSE".to_string(),
            MarketKind::Ose => "OSE".to_string(),
        }
    }
}
