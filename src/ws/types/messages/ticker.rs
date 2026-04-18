use serde::Deserialize;
use std::borrow::Cow;

/// Ticker channel message (type: "ticker")
#[derive(Debug, Clone, Deserialize)]
pub struct WsTicker {
    pub market_ticker: String,
    pub market_id: String,
    pub price_dollars: String,
    pub yes_bid_dollars: String,
    pub yes_ask_dollars: String,
    pub yes_bid_size_fp: String,
    pub yes_ask_size_fp: String,
    pub last_trade_size_fp: String,
    pub volume_fp: String,
    pub open_interest_fp: String,
    pub dollar_volume: i64,
    pub dollar_open_interest: i64,
    pub ts: i64,
    pub ts_ms: i64,
    pub time: String,
}

/// Ticker channel message (type: "ticker")
#[derive(Debug, Clone, Deserialize)]
pub struct WsTickerRef<'a> {
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub market_id: Cow<'a, str>,
    #[serde(borrow)]
    pub price_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_bid_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_ask_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_bid_size_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_ask_size_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub last_trade_size_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub volume_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub open_interest_fp: Cow<'a, str>,
    pub dollar_volume: i64,
    pub dollar_open_interest: i64,
    pub ts: i64,
    pub ts_ms: i64,
    #[serde(borrow)]
    pub time: Cow<'a, str>,
}

impl<'a> WsTickerRef<'a> {
    pub fn into_owned(self) -> WsTicker {
        WsTicker {
            market_ticker: self.market_ticker.into_owned(),
            market_id: self.market_id.into_owned(),
            price_dollars: self.price_dollars.into_owned(),
            yes_bid_dollars: self.yes_bid_dollars.into_owned(),
            yes_ask_dollars: self.yes_ask_dollars.into_owned(),
            yes_bid_size_fp: self.yes_bid_size_fp.into_owned(),
            yes_ask_size_fp: self.yes_ask_size_fp.into_owned(),
            last_trade_size_fp: self.last_trade_size_fp.into_owned(),
            volume_fp: self.volume_fp.into_owned(),
            open_interest_fp: self.open_interest_fp.into_owned(),
            dollar_volume: self.dollar_volume,
            dollar_open_interest: self.dollar_open_interest,
            ts: self.ts,
            ts_ms: self.ts_ms,
            time: self.time.into_owned(),
        }
    }
}
