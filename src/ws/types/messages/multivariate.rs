use crate::types::YesNo;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize)]
pub struct WsMultivariateSelectedMarket {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: YesNo,
}

/// Multivariate message payload (type: "multivariate_lookup")
#[derive(Debug, Clone, Deserialize)]
pub struct WsMultivariate {
    pub collection_ticker: String,
    pub event_ticker: String,
    pub market_ticker: String,
    pub selected_markets: Vec<WsMultivariateSelectedMarket>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsMultivariateSelectedMarketRef<'a> {
    #[serde(borrow)]
    pub event_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    pub side: YesNo,
}

impl<'a> WsMultivariateSelectedMarketRef<'a> {
    pub fn into_owned(self) -> WsMultivariateSelectedMarket {
        WsMultivariateSelectedMarket {
            event_ticker: self.event_ticker.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            side: self.side,
        }
    }
}

/// Multivariate message payload (type: "multivariate_lookup")
#[derive(Debug, Clone, Deserialize)]
pub struct WsMultivariateRef<'a> {
    #[serde(borrow)]
    pub collection_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub event_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub selected_markets: Vec<WsMultivariateSelectedMarketRef<'a>>,
}

impl<'a> WsMultivariateRef<'a> {
    pub fn into_owned(self) -> WsMultivariate {
        WsMultivariate {
            collection_ticker: self.collection_ticker.into_owned(),
            event_ticker: self.event_ticker.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            selected_markets: self
                .selected_markets
                .into_iter()
                .map(WsMultivariateSelectedMarketRef::into_owned)
                .collect(),
        }
    }
}
