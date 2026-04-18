use super::{FixedPointCountRef, FixedPointDollarsRef};
use crate::types::{FixedPointCount, FixedPointDollars};
use serde::Deserialize;
use std::borrow::Cow;

/// Market position message (type: "market_position")
#[derive(Debug, Clone, Deserialize)]
pub struct WsMarketPosition {
    pub user_id: String,
    pub market_ticker: String,
    pub position_fp: FixedPointCount,
    pub position_cost_dollars: FixedPointDollars,
    pub realized_pnl_dollars: FixedPointDollars,
    pub fees_paid_dollars: FixedPointDollars,
    pub position_fee_cost_dollars: FixedPointDollars,
    pub volume_fp: FixedPointCount,
    #[serde(default)]
    pub subaccount: Option<u32>,
}

/// Market position message (type: "market_position")
#[derive(Debug, Clone, Deserialize)]
pub struct WsMarketPositionRef<'a> {
    #[serde(borrow)]
    pub user_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub position_fp: FixedPointCountRef<'a>,
    #[serde(borrow)]
    pub position_cost_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub realized_pnl_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub fees_paid_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub position_fee_cost_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub volume_fp: FixedPointCountRef<'a>,
    #[serde(default)]
    pub subaccount: Option<u32>,
}

impl<'a> WsMarketPositionRef<'a> {
    pub fn into_owned(self) -> WsMarketPosition {
        WsMarketPosition {
            user_id: self.user_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            position_fp: self.position_fp.into_owned(),
            position_cost_dollars: self.position_cost_dollars.into_owned(),
            realized_pnl_dollars: self.realized_pnl_dollars.into_owned(),
            fees_paid_dollars: self.fees_paid_dollars.into_owned(),
            position_fee_cost_dollars: self.position_fee_cost_dollars.into_owned(),
            volume_fp: self.volume_fp.into_owned(),
            subaccount: self.subaccount,
        }
    }
}
