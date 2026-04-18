use crate::error::KalshiError;
use crate::rest::types::{EventPosition, MarketPosition};
use crate::types::{
    BuySell, FixedPointCount, FixedPointDollars, OrderStatus, SelfTradePreventionType,
    TradeTakerSide, YesNo,
};

use bytes::Bytes;
use serde::de::{Error as _, Visitor};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use serde_json::{Map, Value};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsChannelV2 {
    // Public (no auth required)
    Ticker,
    Trade,
    MarketLifecycleV2,
    MultivariateMarketLifecycle,
    Multivariate,

    // Private (auth required)
    OrderbookDelta,
    Fill,
    MarketPositions,
    Communications,
    OrderGroupUpdates,
    UserOrders,
}

impl WsChannelV2 {
    pub fn as_str(self) -> &'static str {
        match self {
            WsChannelV2::Ticker => "ticker",
            WsChannelV2::Trade => "trade",
            WsChannelV2::MarketLifecycleV2 => "market_lifecycle_v2",
            WsChannelV2::MultivariateMarketLifecycle => "multivariate_market_lifecycle",
            WsChannelV2::Multivariate => "multivariate",
            WsChannelV2::OrderbookDelta => "orderbook_delta",
            WsChannelV2::Fill => "fill",
            WsChannelV2::MarketPositions => "market_positions",
            WsChannelV2::Communications => "communications",
            WsChannelV2::OrderGroupUpdates => "order_group_updates",
            WsChannelV2::UserOrders => "user_orders",
        }
    }

    pub fn is_private(self) -> bool {
        matches!(
            self,
            WsChannelV2::OrderbookDelta
                | WsChannelV2::Fill
                | WsChannelV2::MarketPositions
                | WsChannelV2::Communications
                | WsChannelV2::OrderGroupUpdates
                | WsChannelV2::UserOrders
        )
    }
}

impl fmt::Display for WsChannelV2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WsMsgType {
    Subscribed,
    Unsubscribed,
    Ok,
    ListSubscriptions,
    Error,
    Ticker,
    Trade,
    OrderbookSnapshot,
    OrderbookDelta,
    Fill,
    MarketPosition,
    MarketLifecycleV2,
    MultivariateMarketLifecycle,
    EventLifecycle,
    Multivariate,
    MultivariateLookup,
    Communications,
    RfqCreated,
    RfqDeleted,
    QuoteCreated,
    QuoteAccepted,
    QuoteExecuted,
    OrderGroupUpdates,
    UserOrder,
    Unknown(String),
}

impl WsMsgType {
    pub fn as_str(&self) -> &str {
        match self {
            WsMsgType::Subscribed => "subscribed",
            WsMsgType::Unsubscribed => "unsubscribed",
            WsMsgType::Ok => "ok",
            WsMsgType::ListSubscriptions => "list_subscriptions",
            WsMsgType::Error => "error",
            WsMsgType::Ticker => "ticker",
            WsMsgType::Trade => "trade",
            WsMsgType::OrderbookSnapshot => "orderbook_snapshot",
            WsMsgType::OrderbookDelta => "orderbook_delta",
            WsMsgType::Fill => "fill",
            WsMsgType::MarketPosition => "market_position",
            WsMsgType::MarketLifecycleV2 => "market_lifecycle_v2",
            WsMsgType::MultivariateMarketLifecycle => "multivariate_market_lifecycle",
            WsMsgType::EventLifecycle => "event_lifecycle",
            WsMsgType::Multivariate => "multivariate",
            WsMsgType::MultivariateLookup => "multivariate_lookup",
            WsMsgType::Communications => "communications",
            WsMsgType::RfqCreated => "rfq_created",
            WsMsgType::RfqDeleted => "rfq_deleted",
            WsMsgType::QuoteCreated => "quote_created",
            WsMsgType::QuoteAccepted => "quote_accepted",
            WsMsgType::QuoteExecuted => "quote_executed",
            WsMsgType::OrderGroupUpdates => "order_group_updates",
            WsMsgType::UserOrder => "user_order",
            WsMsgType::Unknown(value) => value.as_str(),
        }
    }

    fn from_str(value: &str) -> Option<Self> {
        Some(match value {
            "subscribed" => WsMsgType::Subscribed,
            "unsubscribed" => WsMsgType::Unsubscribed,
            "ok" => WsMsgType::Ok,
            "list_subscriptions" => WsMsgType::ListSubscriptions,
            "error" => WsMsgType::Error,
            "ticker" => WsMsgType::Ticker,
            "trade" => WsMsgType::Trade,
            "orderbook_snapshot" => WsMsgType::OrderbookSnapshot,
            "orderbook_delta" => WsMsgType::OrderbookDelta,
            "fill" => WsMsgType::Fill,
            "market_position" | "market_positions" => WsMsgType::MarketPosition,
            "market_lifecycle_v2" => WsMsgType::MarketLifecycleV2,
            "multivariate_market_lifecycle" => WsMsgType::MultivariateMarketLifecycle,
            "event_lifecycle" | "event_lifecycle_v2" => WsMsgType::EventLifecycle,
            "multivariate" => WsMsgType::Multivariate,
            "multivariate_lookup" => WsMsgType::MultivariateLookup,
            "communications" => WsMsgType::Communications,
            "rfq_created" => WsMsgType::RfqCreated,
            "rfq_deleted" => WsMsgType::RfqDeleted,
            "quote_created" => WsMsgType::QuoteCreated,
            "quote_accepted" => WsMsgType::QuoteAccepted,
            "quote_executed" => WsMsgType::QuoteExecuted,
            "order_group_updates" => WsMsgType::OrderGroupUpdates,
            "user_order" => WsMsgType::UserOrder,
            _ => return None,
        })
    }

    fn from_string(value: String) -> Self {
        match value.as_str() {
            "subscribed" => WsMsgType::Subscribed,
            "unsubscribed" => WsMsgType::Unsubscribed,
            "ok" => WsMsgType::Ok,
            "list_subscriptions" => WsMsgType::ListSubscriptions,
            "error" => WsMsgType::Error,
            "ticker" => WsMsgType::Ticker,
            "trade" => WsMsgType::Trade,
            "orderbook_snapshot" => WsMsgType::OrderbookSnapshot,
            "orderbook_delta" => WsMsgType::OrderbookDelta,
            "fill" => WsMsgType::Fill,
            "market_position" | "market_positions" => WsMsgType::MarketPosition,
            "market_lifecycle_v2" => WsMsgType::MarketLifecycleV2,
            "multivariate_market_lifecycle" => WsMsgType::MultivariateMarketLifecycle,
            "event_lifecycle" | "event_lifecycle_v2" => WsMsgType::EventLifecycle,
            "multivariate" => WsMsgType::Multivariate,
            "multivariate_lookup" => WsMsgType::MultivariateLookup,
            "communications" => WsMsgType::Communications,
            "rfq_created" => WsMsgType::RfqCreated,
            "rfq_deleted" => WsMsgType::RfqDeleted,
            "quote_created" => WsMsgType::QuoteCreated,
            "quote_accepted" => WsMsgType::QuoteAccepted,
            "quote_executed" => WsMsgType::QuoteExecuted,
            "order_group_updates" => WsMsgType::OrderGroupUpdates,
            "user_order" => WsMsgType::UserOrder,
            _ => WsMsgType::Unknown(value),
        }
    }
}

impl fmt::Display for WsMsgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for WsMsgType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for WsMsgType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct WsMsgTypeVisitor;

        impl<'de> Visitor<'de> for WsMsgTypeVisitor {
            type Value = WsMsgType;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a websocket message type string")
            }

            fn visit_borrowed_str<E: serde::de::Error>(
                self,
                value: &'de str,
            ) -> Result<Self::Value, E> {
                Ok(WsMsgType::from_str(value)
                    .unwrap_or_else(|| WsMsgType::Unknown(value.to_owned())))
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                Ok(WsMsgType::from_str(value)
                    .unwrap_or_else(|| WsMsgType::Unknown(value.to_owned())))
            }

            fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
                Ok(WsMsgType::from_string(value))
            }
        }

        deserializer.deserialize_str(WsMsgTypeVisitor)
    }
}

/// Subscription parameters for WebSocket channels.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WsSubscriptionParamsV2 {
    pub channels: Vec<WsChannelV2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tickers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_initial_snapshot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_ticker_ack: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_factor: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_key: Option<u32>,
}

impl WsSubscriptionParamsV2 {
    /// Collect all market tickers from both singular and plural fields.
    pub fn all_market_tickers(&self) -> Vec<&str> {
        let mut out = Vec::new();
        if let Some(single) = &self.market_ticker {
            out.push(single.as_str());
        }
        if let Some(multi) = &self.market_tickers {
            out.extend(multi.iter().map(String::as_str));
        }
        out
    }

    /// Collect all market IDs from both singular and plural fields.
    pub fn all_market_ids(&self) -> Vec<&str> {
        let mut out = Vec::new();
        if let Some(single) = &self.market_id {
            out.push(single.as_str());
        }
        if let Some(multi) = &self.market_ids {
            out.extend(multi.iter().map(String::as_str));
        }
        out
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsSubscriptionInfo {
    pub sid: u64,
    #[serde(default)]
    pub channels: Vec<WsChannelV2>,
    #[serde(default)]
    pub channel: Option<WsChannelV2>,
    #[serde(default)]
    pub market_tickers: Option<Vec<String>>,
    #[serde(default)]
    pub market_ids: Option<Vec<String>>,
    #[serde(default)]
    pub event_tickers: Option<Vec<String>>,
    #[serde(default)]
    pub send_initial_snapshot: Option<bool>,
    #[serde(default)]
    pub skip_ticker_ack: Option<bool>,
    #[serde(default)]
    pub shard_factor: Option<u32>,
    #[serde(default)]
    pub shard_key: Option<String>,
}

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

/// Trade channel message (type: "trade")
#[derive(Debug, Clone, Deserialize)]
pub struct WsTrade {
    pub trade_id: String,
    #[serde(alias = "ticker")]
    pub market_ticker: String,
    pub count_fp: String,
    pub yes_price_dollars: String,
    pub no_price_dollars: String,
    pub taker_side: TradeTakerSide,
    pub ts: i64,
    pub ts_ms: i64,
    #[serde(default)]
    pub created_time: Option<String>,
}

/// Orderbook snapshot message (type: "orderbook_snapshot")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderbookSnapshot {
    pub market_ticker: String,
    pub market_id: String,
    /// Price levels: (price_dollars, quantity_fp) - fully fixed-point
    #[serde(default)]
    pub yes_dollars_fp: Vec<(String, String)>,
    /// Price levels: (price_dollars, quantity_fp) - fully fixed-point
    #[serde(default)]
    pub no_dollars_fp: Vec<(String, String)>,
}

/// Orderbook delta message (type: "orderbook_delta")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderbookDelta {
    pub market_ticker: String,
    pub market_id: String,
    pub price_dollars: String,
    pub delta_fp: String,
    pub side: YesNo,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub subaccount: Option<i64>,
    #[serde(default)]
    pub ts: Option<String>,
    #[serde(default)]
    pub ts_ms: Option<i64>,
}

/// Fill channel message (type: "fill")
#[derive(Debug, Clone, Deserialize)]
pub struct WsFill {
    pub trade_id: String,
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(alias = "ticker")]
    pub market_ticker: String,
    pub side: YesNo,
    pub action: BuySell,
    pub count_fp: String,
    pub yes_price_dollars: String,
    pub is_taker: bool,
    pub fee_cost: String,
    pub ts: i64,
    pub ts_ms: i64,
    pub post_position_fp: String,
    pub purchased_side: YesNo,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    #[serde(alias = "subaccount_number")]
    pub subaccount: Option<i64>,
}

/// Market lifecycle message (type: "market_lifecycle_v2")
#[derive(Debug, Clone, Deserialize)]
pub struct WsMarketLifecycleV2 {
    pub market_ticker: String,
    #[serde(default)]
    pub event_type: Option<WsMarketLifecycleEventType>,
    #[serde(default)]
    pub open_ts: Option<i64>,
    #[serde(default)]
    pub close_ts: Option<i64>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub determination_ts: Option<i64>,
    #[serde(default)]
    pub settlement_value: Option<String>,
    #[serde(default)]
    pub settled_ts: Option<i64>,
    #[serde(default)]
    pub is_deactivated: Option<bool>,
    #[serde(default)]
    pub fractional_trading_enabled: Option<bool>,
    #[serde(default)]
    pub price_level_structure: Option<String>,
    #[serde(default)]
    pub additional_metadata: Option<WsMarketLifecycleAdditionalMetadata>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WsMarketLifecycleEventType {
    Created,
    Activated,
    Deactivated,
    CloseDateUpdated,
    Determined,
    Settled,
    FractionalTradingUpdated,
    PriceLevelStructureUpdated,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsMarketLifecycleAdditionalMetadata {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub yes_sub_title: Option<String>,
    #[serde(default)]
    pub no_sub_title: Option<String>,
    #[serde(default)]
    pub rules_primary: Option<String>,
    #[serde(default)]
    pub rules_secondary: Option<String>,
    #[serde(default)]
    pub can_close_early: Option<bool>,
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub expected_expiration_ts: Option<i64>,
    #[serde(default)]
    pub strike_type: Option<String>,
    #[serde(default)]
    pub floor_strike: Option<f64>,
    #[serde(default)]
    pub cap_strike: Option<f64>,
    #[serde(default)]
    pub custom_strike: Option<BTreeMap<String, String>>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

/// Event lifecycle message (type: "event_lifecycle")
#[derive(Debug, Clone, Deserialize)]
pub struct WsEventLifecycle {
    pub event_ticker: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub collateral_return_type: Option<String>,
    #[serde(default)]
    pub series_ticker: Option<String>,
    #[serde(default)]
    pub strike_date: Option<i64>,
    #[serde(default)]
    pub strike_period: Option<String>,
    #[serde(default)]
    pub additional_metadata: Option<WsEventLifecycleAdditionalMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsEventLifecycleAdditionalMetadata {
    #[serde(default)]
    pub custom_strike: Option<BTreeMap<String, String>>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

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

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsOrderGroupEventType {
    Created,
    Triggered,
    Reset,
    Deleted,
    LimitUpdated,
    #[serde(other)]
    Unknown,
}

/// Order group update message payload (type: "order_group_updates")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderGroupUpdate {
    pub event_type: WsOrderGroupEventType,
    pub order_group_id: String,
    #[serde(default)]
    pub contracts_limit_fp: Option<FixedPointCount>,
}

/// User order update payload (type: "user_order").
#[derive(Debug, Clone, Deserialize)]
pub struct WsUserOrder {
    pub order_id: String,
    pub user_id: String,
    pub ticker: String,
    #[serde(default)]
    pub status: Option<OrderStatus>,
    #[serde(default)]
    pub side: Option<YesNo>,
    #[serde(default)]
    pub is_yes: Option<bool>,
    #[serde(default)]
    pub yes_price_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub fill_count_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub remaining_count_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub initial_count_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub taker_fill_cost_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub maker_fill_cost_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub taker_fees_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub maker_fees_dollars: Option<FixedPointDollars>,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub order_group_id: Option<String>,
    #[serde(default)]
    pub self_trade_prevention_type: Option<SelfTradePreventionType>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub created_ts_ms: Option<i64>,
    #[serde(default)]
    pub last_update_time: Option<String>,
    #[serde(default)]
    pub last_updated_ts_ms: Option<i64>,
    #[serde(default)]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub expiration_ts_ms: Option<i64>,
    #[serde(default)]
    pub subaccount_number: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsMveSelectedLeg {
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub market_ticker: Option<String>,
    #[serde(default)]
    pub side: Option<YesNo>,
    #[serde(default)]
    pub yes_settlement_value_dollars: Option<FixedPointDollars>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqCreated {
    pub id: String,
    pub creator_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub contracts_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub target_cost_dollars: Option<FixedPointDollars>,
    pub created_ts: String,
    #[serde(default)]
    pub mve_collection_ticker: Option<String>,
    #[serde(default)]
    pub mve_selected_legs: Option<Vec<WsMveSelectedLeg>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqDeleted {
    pub id: String,
    pub creator_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    #[serde(default)]
    pub contracts_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub target_cost_dollars: Option<FixedPointDollars>,
    pub deleted_ts: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteCreated {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: FixedPointDollars,
    pub no_bid_dollars: FixedPointDollars,
    #[serde(default)]
    pub yes_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub no_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub rfq_target_cost_dollars: Option<FixedPointDollars>,
    pub created_ts: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteAccepted {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub event_ticker: Option<String>,
    pub yes_bid_dollars: FixedPointDollars,
    pub no_bid_dollars: FixedPointDollars,
    #[serde(default)]
    pub accepted_side: Option<YesNo>,
    #[serde(default)]
    pub contracts_accepted_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub yes_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub no_contracts_offered_fp: Option<FixedPointCount>,
    #[serde(default)]
    pub rfq_target_cost_dollars: Option<FixedPointDollars>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteExecuted {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub rfq_creator_id: String,
    pub order_id: String,
    pub client_order_id: String,
    pub market_ticker: String,
    pub executed_ts: String,
}

/// Communications message payloads (RFQs and quotes).
#[derive(Debug, Clone)]
pub enum WsCommunications {
    RfqCreated(WsRfqCreated),
    RfqDeleted(WsRfqDeleted),
    QuoteCreated(WsQuoteCreated),
    QuoteAccepted(WsQuoteAccepted),
    QuoteExecuted(WsQuoteExecuted),
}

/// Borrowed fixed-point dollar string (e.g. "0.5600").
pub type FixedPointDollarsRef<'a> = Cow<'a, str>;

/// Borrowed fixed-point contract count string (e.g. "10.00").
pub type FixedPointCountRef<'a> = Cow<'a, str>;

#[derive(Debug, Clone, Deserialize)]
pub struct MarketPositionRef<'a> {
    #[serde(borrow)]
    pub ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub total_traded_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub position_fp: FixedPointCountRef<'a>,
    #[serde(borrow)]
    pub market_exposure_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub realized_pnl_dollars: FixedPointDollarsRef<'a>,
    #[serde(default)]
    pub resting_orders_count: Option<i32>,
    #[serde(borrow)]
    pub fees_paid_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub last_updated_ts: Cow<'a, str>,
}

impl<'a> MarketPositionRef<'a> {
    pub fn into_owned(self) -> MarketPosition {
        MarketPosition {
            ticker: self.ticker.into_owned(),
            total_traded_dollars: self.total_traded_dollars.into_owned(),
            position_fp: self.position_fp.into_owned(),
            market_exposure_dollars: self.market_exposure_dollars.into_owned(),
            realized_pnl_dollars: self.realized_pnl_dollars.into_owned(),
            resting_orders_count: self.resting_orders_count,
            fees_paid_dollars: self.fees_paid_dollars.into_owned(),
            last_updated_ts: self.last_updated_ts.into_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventPositionRef<'a> {
    #[serde(borrow)]
    pub event_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub total_cost_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub total_cost_shares_fp: FixedPointCountRef<'a>,
    #[serde(borrow)]
    pub event_exposure_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub realized_pnl_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub fees_paid_dollars: FixedPointDollarsRef<'a>,
}

impl<'a> EventPositionRef<'a> {
    pub fn into_owned(self) -> EventPosition {
        EventPosition {
            event_ticker: self.event_ticker.into_owned(),
            total_cost_dollars: self.total_cost_dollars.into_owned(),
            total_cost_shares_fp: self.total_cost_shares_fp.into_owned(),
            event_exposure_dollars: self.event_exposure_dollars.into_owned(),
            realized_pnl_dollars: self.realized_pnl_dollars.into_owned(),
            fees_paid_dollars: self.fees_paid_dollars.into_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsSubscriptionInfoRef<'a> {
    pub sid: u64,
    #[serde(default)]
    pub channels: Vec<WsChannelV2>,
    #[serde(default)]
    pub channel: Option<WsChannelV2>,
    #[serde(default, borrow)]
    pub market_tickers: Option<Vec<Cow<'a, str>>>,
    #[serde(default, borrow)]
    pub market_ids: Option<Vec<Cow<'a, str>>>,
    #[serde(default, borrow)]
    pub event_tickers: Option<Vec<Cow<'a, str>>>,
    #[serde(default)]
    pub send_initial_snapshot: Option<bool>,
    #[serde(default)]
    pub skip_ticker_ack: Option<bool>,
    #[serde(default)]
    pub shard_factor: Option<u32>,
    #[serde(default, borrow)]
    pub shard_key: Option<Cow<'a, str>>,
}

impl<'a> WsSubscriptionInfoRef<'a> {
    pub fn into_owned(self) -> WsSubscriptionInfo {
        WsSubscriptionInfo {
            sid: self.sid,
            channels: self.channels,
            channel: self.channel,
            market_tickers: self
                .market_tickers
                .map(|v| v.into_iter().map(Cow::into_owned).collect()),
            market_ids: self
                .market_ids
                .map(|v| v.into_iter().map(Cow::into_owned).collect()),
            event_tickers: self
                .event_tickers
                .map(|v| v.into_iter().map(Cow::into_owned).collect()),
            send_initial_snapshot: self.send_initial_snapshot,
            skip_ticker_ack: self.skip_ticker_ack,
            shard_factor: self.shard_factor,
            shard_key: self.shard_key.map(Cow::into_owned),
        }
    }
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

/// Trade channel message (type: "trade")
#[derive(Debug, Clone, Deserialize)]
pub struct WsTradeRef<'a> {
    #[serde(borrow)]
    pub trade_id: Cow<'a, str>,
    #[serde(alias = "ticker", borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub count_fp: Cow<'a, str>,
    #[serde(borrow)]
    pub yes_price_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub no_price_dollars: Cow<'a, str>,
    pub taker_side: TradeTakerSide,
    pub ts: i64,
    pub ts_ms: i64,
    #[serde(default, borrow)]
    pub created_time: Option<Cow<'a, str>>,
}

impl<'a> WsTradeRef<'a> {
    pub fn into_owned(self) -> WsTrade {
        WsTrade {
            trade_id: self.trade_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            count_fp: self.count_fp.into_owned(),
            yes_price_dollars: self.yes_price_dollars.into_owned(),
            no_price_dollars: self.no_price_dollars.into_owned(),
            taker_side: self.taker_side,
            ts: self.ts,
            ts_ms: self.ts_ms,
            created_time: self.created_time.map(Cow::into_owned),
        }
    }
}

/// Orderbook snapshot message (type: "orderbook_snapshot")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderbookSnapshotRef<'a> {
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub market_id: Cow<'a, str>,
    /// Price levels: (price_dollars, quantity_fp) - fully fixed-point
    #[serde(default, borrow)]
    pub yes_dollars_fp: Vec<(Cow<'a, str>, Cow<'a, str>)>,
    /// Price levels: (price_dollars, quantity_fp) - fully fixed-point
    #[serde(default, borrow)]
    pub no_dollars_fp: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> WsOrderbookSnapshotRef<'a> {
    pub fn into_owned(self) -> WsOrderbookSnapshot {
        WsOrderbookSnapshot {
            market_ticker: self.market_ticker.into_owned(),
            market_id: self.market_id.into_owned(),
            yes_dollars_fp: self
                .yes_dollars_fp
                .into_iter()
                .map(|(p, q)| (p.into_owned(), q.into_owned()))
                .collect(),
            no_dollars_fp: self
                .no_dollars_fp
                .into_iter()
                .map(|(p, q)| (p.into_owned(), q.into_owned()))
                .collect(),
        }
    }
}

/// Orderbook delta message (type: "orderbook_delta")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderbookDeltaRef<'a> {
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub market_id: Cow<'a, str>,
    #[serde(borrow)]
    pub price_dollars: Cow<'a, str>,
    #[serde(borrow)]
    pub delta_fp: Cow<'a, str>,
    pub side: YesNo,
    #[serde(default, borrow)]
    pub client_order_id: Option<Cow<'a, str>>,
    #[serde(default)]
    pub subaccount: Option<i64>,
    #[serde(default, borrow)]
    pub ts: Option<Cow<'a, str>>,
    #[serde(default)]
    pub ts_ms: Option<i64>,
}

impl<'a> WsOrderbookDeltaRef<'a> {
    pub fn into_owned(self) -> WsOrderbookDelta {
        WsOrderbookDelta {
            market_ticker: self.market_ticker.into_owned(),
            market_id: self.market_id.into_owned(),
            price_dollars: self.price_dollars.into_owned(),
            delta_fp: self.delta_fp.into_owned(),
            side: self.side,
            client_order_id: self.client_order_id.map(Cow::into_owned),
            subaccount: self.subaccount,
            ts: self.ts.map(Cow::into_owned),
            ts_ms: self.ts_ms,
        }
    }
}

/// Fill channel message (type: "fill")
#[derive(Debug, Clone, Deserialize)]
pub struct WsFillRef<'a> {
    #[serde(borrow)]
    pub trade_id: Cow<'a, str>,
    #[serde(borrow)]
    pub order_id: Cow<'a, str>,
    #[serde(default, borrow)]
    pub client_order_id: Option<Cow<'a, str>>,
    #[serde(alias = "ticker", borrow)]
    pub market_ticker: Cow<'a, str>,
    pub side: YesNo,
    pub action: BuySell,
    #[serde(borrow)]
    pub count_fp: Cow<'a, str>,
    pub yes_price_dollars: Cow<'a, str>,
    pub is_taker: bool,
    #[serde(borrow)]
    pub fee_cost: Cow<'a, str>,
    pub ts: i64,
    pub ts_ms: i64,
    #[serde(borrow)]
    pub post_position_fp: Cow<'a, str>,
    pub purchased_side: YesNo,
    #[serde(default, borrow)]
    pub created_time: Option<Cow<'a, str>>,
    #[serde(default)]
    #[serde(alias = "subaccount_number")]
    pub subaccount: Option<i64>,
}

impl<'a> WsFillRef<'a> {
    pub fn into_owned(self) -> WsFill {
        WsFill {
            trade_id: self.trade_id.into_owned(),
            order_id: self.order_id.into_owned(),
            client_order_id: self.client_order_id.map(Cow::into_owned),
            market_ticker: self.market_ticker.into_owned(),
            side: self.side,
            action: self.action,
            count_fp: self.count_fp.into_owned(),
            yes_price_dollars: self.yes_price_dollars.into_owned(),
            is_taker: self.is_taker,
            fee_cost: self.fee_cost.into_owned(),
            ts: self.ts,
            ts_ms: self.ts_ms,
            post_position_fp: self.post_position_fp.into_owned(),
            purchased_side: self.purchased_side,
            created_time: self.created_time.map(Cow::into_owned),
            subaccount: self.subaccount,
        }
    }
}

/// Market lifecycle message (type: "market_lifecycle_v2")
#[derive(Debug, Clone, Deserialize)]
pub struct WsMarketLifecycleV2Ref<'a> {
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(default)]
    pub event_type: Option<WsMarketLifecycleEventType>,
    #[serde(default)]
    pub open_ts: Option<i64>,
    #[serde(default)]
    pub close_ts: Option<i64>,
    #[serde(default, borrow)]
    pub result: Option<Cow<'a, str>>,
    #[serde(default)]
    pub determination_ts: Option<i64>,
    #[serde(default, borrow)]
    pub settlement_value: Option<Cow<'a, str>>,
    #[serde(default)]
    pub settled_ts: Option<i64>,
    #[serde(default)]
    pub is_deactivated: Option<bool>,
    #[serde(default)]
    pub fractional_trading_enabled: Option<bool>,
    #[serde(default, borrow)]
    pub price_level_structure: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub additional_metadata: Option<WsMarketLifecycleAdditionalMetadataRef<'a>>,
}

impl<'a> WsMarketLifecycleV2Ref<'a> {
    pub fn into_owned(self) -> WsMarketLifecycleV2 {
        WsMarketLifecycleV2 {
            market_ticker: self.market_ticker.into_owned(),
            event_type: self.event_type,
            open_ts: self.open_ts,
            close_ts: self.close_ts,
            result: self.result.map(Cow::into_owned),
            determination_ts: self.determination_ts,
            settlement_value: self.settlement_value.map(Cow::into_owned),
            settled_ts: self.settled_ts,
            is_deactivated: self.is_deactivated,
            fractional_trading_enabled: self.fractional_trading_enabled,
            price_level_structure: self.price_level_structure.map(Cow::into_owned),
            additional_metadata: self
                .additional_metadata
                .map(WsMarketLifecycleAdditionalMetadataRef::into_owned),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsMarketLifecycleAdditionalMetadataRef<'a> {
    #[serde(default, borrow)]
    pub name: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub title: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub yes_sub_title: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub no_sub_title: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub rules_primary: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub rules_secondary: Option<Cow<'a, str>>,
    #[serde(default)]
    pub can_close_early: Option<bool>,
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(default)]
    pub expected_expiration_ts: Option<i64>,
    #[serde(default, borrow)]
    pub strike_type: Option<Cow<'a, str>>,
    #[serde(default)]
    pub floor_strike: Option<f64>,
    #[serde(default)]
    pub cap_strike: Option<f64>,
    #[serde(default)]
    pub custom_strike: Option<BTreeMap<String, String>>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

impl<'a> WsMarketLifecycleAdditionalMetadataRef<'a> {
    pub fn into_owned(self) -> WsMarketLifecycleAdditionalMetadata {
        WsMarketLifecycleAdditionalMetadata {
            name: self.name.map(Cow::into_owned),
            title: self.title.map(Cow::into_owned),
            yes_sub_title: self.yes_sub_title.map(Cow::into_owned),
            no_sub_title: self.no_sub_title.map(Cow::into_owned),
            rules_primary: self.rules_primary.map(Cow::into_owned),
            rules_secondary: self.rules_secondary.map(Cow::into_owned),
            can_close_early: self.can_close_early,
            event_ticker: self.event_ticker.map(Cow::into_owned),
            expected_expiration_ts: self.expected_expiration_ts,
            strike_type: self.strike_type.map(Cow::into_owned),
            floor_strike: self.floor_strike,
            cap_strike: self.cap_strike,
            custom_strike: self.custom_strike,
            extra: self.extra,
        }
    }
}

/// Event lifecycle message (type: "event_lifecycle")
#[derive(Debug, Clone, Deserialize)]
pub struct WsEventLifecycleRef<'a> {
    #[serde(borrow)]
    pub event_ticker: Cow<'a, str>,
    #[serde(default, borrow)]
    pub title: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub subtitle: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub collateral_return_type: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub series_ticker: Option<Cow<'a, str>>,
    #[serde(default)]
    pub strike_date: Option<i64>,
    #[serde(default, borrow)]
    pub strike_period: Option<Cow<'a, str>>,
    #[serde(default)]
    pub additional_metadata: Option<WsEventLifecycleAdditionalMetadataRef>,
}

impl<'a> WsEventLifecycleRef<'a> {
    pub fn into_owned(self) -> WsEventLifecycle {
        WsEventLifecycle {
            event_ticker: self.event_ticker.into_owned(),
            title: self.title.map(Cow::into_owned),
            subtitle: self.subtitle.map(Cow::into_owned),
            collateral_return_type: self.collateral_return_type.map(Cow::into_owned),
            series_ticker: self.series_ticker.map(Cow::into_owned),
            strike_date: self.strike_date,
            strike_period: self.strike_period.map(Cow::into_owned),
            additional_metadata: self
                .additional_metadata
                .map(WsEventLifecycleAdditionalMetadataRef::into_owned),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsEventLifecycleAdditionalMetadataRef {
    #[serde(default)]
    pub custom_strike: Option<BTreeMap<String, String>>,
    #[serde(default, flatten)]
    pub extra: Map<String, Value>,
}

impl WsEventLifecycleAdditionalMetadataRef {
    pub fn into_owned(self) -> WsEventLifecycleAdditionalMetadata {
        WsEventLifecycleAdditionalMetadata {
            custom_strike: self.custom_strike,
            extra: self.extra,
        }
    }
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

/// Order group update message payload (type: "order_group_updates")
#[derive(Debug, Clone, Deserialize)]
pub struct WsOrderGroupUpdateRef<'a> {
    pub event_type: WsOrderGroupEventType,
    #[serde(borrow)]
    pub order_group_id: Cow<'a, str>,
    #[serde(default, borrow)]
    pub contracts_limit_fp: Option<FixedPointCountRef<'a>>,
}

impl<'a> WsOrderGroupUpdateRef<'a> {
    pub fn into_owned(self) -> WsOrderGroupUpdate {
        WsOrderGroupUpdate {
            event_type: self.event_type,
            order_group_id: self.order_group_id.into_owned(),
            contracts_limit_fp: self.contracts_limit_fp.map(Cow::into_owned),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsMveSelectedLegRef<'a> {
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub market_ticker: Option<Cow<'a, str>>,
    #[serde(default)]
    pub side: Option<YesNo>,
    #[serde(default, borrow)]
    pub yes_settlement_value_dollars: Option<FixedPointDollarsRef<'a>>,
}

impl<'a> WsMveSelectedLegRef<'a> {
    pub fn into_owned(self) -> WsMveSelectedLeg {
        WsMveSelectedLeg {
            event_ticker: self.event_ticker.map(Cow::into_owned),
            market_ticker: self.market_ticker.map(Cow::into_owned),
            side: self.side,
            yes_settlement_value_dollars: self.yes_settlement_value_dollars.map(Cow::into_owned),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqCreatedRef<'a> {
    #[serde(borrow)]
    pub id: Cow<'a, str>,
    #[serde(borrow)]
    pub creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub contracts_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub target_cost_dollars: Option<FixedPointDollarsRef<'a>>,
    #[serde(borrow)]
    pub created_ts: Cow<'a, str>,
    #[serde(default, borrow)]
    pub mve_collection_ticker: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub mve_selected_legs: Option<Vec<WsMveSelectedLegRef<'a>>>,
}

impl<'a> WsRfqCreatedRef<'a> {
    pub fn into_owned(self) -> WsRfqCreated {
        WsRfqCreated {
            id: self.id.into_owned(),
            creator_id: self.creator_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            contracts_fp: self.contracts_fp.map(Cow::into_owned),
            target_cost_dollars: self.target_cost_dollars.map(Cow::into_owned),
            created_ts: self.created_ts.into_owned(),
            mve_collection_ticker: self.mve_collection_ticker.map(Cow::into_owned),
            mve_selected_legs: self.mve_selected_legs.map(|legs| {
                legs.into_iter()
                    .map(WsMveSelectedLegRef::into_owned)
                    .collect()
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsRfqDeletedRef<'a> {
    #[serde(borrow)]
    pub id: Cow<'a, str>,
    #[serde(borrow)]
    pub creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    pub contracts_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub target_cost_dollars: Option<FixedPointDollarsRef<'a>>,
    #[serde(borrow)]
    pub deleted_ts: Cow<'a, str>,
}

impl<'a> WsRfqDeletedRef<'a> {
    pub fn into_owned(self) -> WsRfqDeleted {
        WsRfqDeleted {
            id: self.id.into_owned(),
            creator_id: self.creator_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            contracts_fp: self.contracts_fp.map(Cow::into_owned),
            target_cost_dollars: self.target_cost_dollars.map(Cow::into_owned),
            deleted_ts: self.deleted_ts.into_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteCreatedRef<'a> {
    #[serde(borrow)]
    pub quote_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_id: Cow<'a, str>,
    #[serde(borrow)]
    pub quote_creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub yes_bid_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub no_bid_dollars: FixedPointDollarsRef<'a>,
    #[serde(default, borrow)]
    pub yes_contracts_offered_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub no_contracts_offered_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub rfq_target_cost_dollars: Option<FixedPointDollarsRef<'a>>,
    #[serde(borrow)]
    pub created_ts: Cow<'a, str>,
}

impl<'a> WsQuoteCreatedRef<'a> {
    pub fn into_owned(self) -> WsQuoteCreated {
        WsQuoteCreated {
            quote_id: self.quote_id.into_owned(),
            rfq_id: self.rfq_id.into_owned(),
            quote_creator_id: self.quote_creator_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            yes_bid_dollars: self.yes_bid_dollars.into_owned(),
            no_bid_dollars: self.no_bid_dollars.into_owned(),
            yes_contracts_offered_fp: self.yes_contracts_offered_fp.map(Cow::into_owned),
            no_contracts_offered_fp: self.no_contracts_offered_fp.map(Cow::into_owned),
            rfq_target_cost_dollars: self.rfq_target_cost_dollars.map(Cow::into_owned),
            created_ts: self.created_ts.into_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteAcceptedRef<'a> {
    #[serde(borrow)]
    pub quote_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_id: Cow<'a, str>,
    #[serde(borrow)]
    pub quote_creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(default, borrow)]
    pub event_ticker: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub yes_bid_dollars: FixedPointDollarsRef<'a>,
    #[serde(borrow)]
    pub no_bid_dollars: FixedPointDollarsRef<'a>,
    #[serde(default)]
    pub accepted_side: Option<YesNo>,
    #[serde(default, borrow)]
    pub contracts_accepted_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub yes_contracts_offered_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub no_contracts_offered_fp: Option<FixedPointCountRef<'a>>,
    #[serde(default, borrow)]
    pub rfq_target_cost_dollars: Option<FixedPointDollarsRef<'a>>,
}

impl<'a> WsQuoteAcceptedRef<'a> {
    pub fn into_owned(self) -> WsQuoteAccepted {
        WsQuoteAccepted {
            quote_id: self.quote_id.into_owned(),
            rfq_id: self.rfq_id.into_owned(),
            quote_creator_id: self.quote_creator_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            event_ticker: self.event_ticker.map(Cow::into_owned),
            yes_bid_dollars: self.yes_bid_dollars.into_owned(),
            no_bid_dollars: self.no_bid_dollars.into_owned(),
            accepted_side: self.accepted_side,
            contracts_accepted_fp: self.contracts_accepted_fp.map(Cow::into_owned),
            yes_contracts_offered_fp: self.yes_contracts_offered_fp.map(Cow::into_owned),
            no_contracts_offered_fp: self.no_contracts_offered_fp.map(Cow::into_owned),
            rfq_target_cost_dollars: self.rfq_target_cost_dollars.map(Cow::into_owned),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsQuoteExecutedRef<'a> {
    #[serde(borrow)]
    pub quote_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_id: Cow<'a, str>,
    #[serde(borrow)]
    pub quote_creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub rfq_creator_id: Cow<'a, str>,
    #[serde(borrow)]
    pub order_id: Cow<'a, str>,
    #[serde(borrow)]
    pub client_order_id: Cow<'a, str>,
    #[serde(borrow)]
    pub market_ticker: Cow<'a, str>,
    #[serde(borrow)]
    pub executed_ts: Cow<'a, str>,
}

impl<'a> WsQuoteExecutedRef<'a> {
    pub fn into_owned(self) -> WsQuoteExecuted {
        WsQuoteExecuted {
            quote_id: self.quote_id.into_owned(),
            rfq_id: self.rfq_id.into_owned(),
            quote_creator_id: self.quote_creator_id.into_owned(),
            rfq_creator_id: self.rfq_creator_id.into_owned(),
            order_id: self.order_id.into_owned(),
            client_order_id: self.client_order_id.into_owned(),
            market_ticker: self.market_ticker.into_owned(),
            executed_ts: self.executed_ts.into_owned(),
        }
    }
}

/// Communications message payloads (RFQs and quotes).
#[derive(Debug, Clone)]
pub enum WsCommunicationsRef<'a> {
    RfqCreated(WsRfqCreatedRef<'a>),
    RfqDeleted(WsRfqDeletedRef<'a>),
    QuoteCreated(WsQuoteCreatedRef<'a>),
    QuoteAccepted(WsQuoteAcceptedRef<'a>),
    QuoteExecuted(WsQuoteExecutedRef<'a>),
}

impl<'a> WsCommunicationsRef<'a> {
    pub fn into_owned(self) -> WsCommunications {
        match self {
            WsCommunicationsRef::RfqCreated(msg) => WsCommunications::RfqCreated(msg.into_owned()),
            WsCommunicationsRef::RfqDeleted(msg) => WsCommunications::RfqDeleted(msg.into_owned()),
            WsCommunicationsRef::QuoteCreated(msg) => {
                WsCommunications::QuoteCreated(msg.into_owned())
            }
            WsCommunicationsRef::QuoteAccepted(msg) => {
                WsCommunications::QuoteAccepted(msg.into_owned())
            }
            WsCommunicationsRef::QuoteExecuted(msg) => {
                WsCommunications::QuoteExecuted(msg.into_owned())
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsListSubscriptionsRef<'a> {
    #[serde(default, borrow)]
    pub subscriptions: Vec<WsSubscriptionInfoRef<'a>>,
}

impl<'a> WsListSubscriptionsRef<'a> {
    pub fn into_owned(self) -> WsListSubscriptions {
        WsListSubscriptions {
            subscriptions: self
                .subscriptions
                .into_iter()
                .map(WsSubscriptionInfoRef::into_owned)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsErrorRef<'a> {
    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default, borrow)]
    pub message: Option<Cow<'a, str>>,
}

impl<'a> WsErrorRef<'a> {
    pub fn into_owned(self) -> WsError {
        WsError {
            code: self.code,
            message: self.message.map(Cow::into_owned),
        }
    }
}

/// Envelope used by Kalshi WS (data + errors use "type")
#[derive(Debug, Clone, Deserialize)]
pub struct WsEnvelope {
    pub id: Option<u64>,
    #[serde(rename = "type")]
    pub msg_type: WsMsgType,
    pub sid: Option<u64>,
    pub seq: Option<u64>,
    pub msg: Option<Box<RawValue>>,
    #[serde(default)]
    pub subscriptions: Option<Vec<WsSubscriptionInfo>>,
}

impl WsEnvelope {
    pub fn msg_raw(&self) -> Option<&str> {
        self.msg.as_deref().map(|raw| raw.get())
    }

    pub fn into_message(self) -> Result<WsMessageV2, KalshiError> {
        fn parse_msg<T: for<'de> Deserialize<'de>>(
            msg: &Option<Box<RawValue>>,
        ) -> Result<T, serde_json::Error> {
            let raw = msg
                .as_deref()
                .ok_or_else(|| serde_json::Error::custom("missing msg"))?;
            serde_json::from_str(raw.get())
        }

        #[derive(Deserialize)]
        struct SubscribedMsg {
            #[allow(dead_code)]
            channel: Option<WsChannelV2>,
            sid: Option<u64>,
        }

        let WsEnvelope {
            id,
            msg_type,
            sid,
            seq,
            msg,
            subscriptions,
        } = self;

        match msg_type {
            WsMsgType::Subscribed => {
                let sid = sid.or_else(|| {
                    parse_msg::<SubscribedMsg>(&msg)
                        .ok()
                        .and_then(|value| value.sid)
                });
                Ok(WsMessageV2::Subscribed { id, sid })
            }
            WsMsgType::Unsubscribed => Ok(WsMessageV2::Unsubscribed { id, sid }),
            WsMsgType::Ok => {
                if msg.is_some()
                    && let Ok(subscriptions) = parse_msg::<Vec<WsSubscriptionInfo>>(&msg)
                {
                    return Ok(WsMessageV2::ListSubscriptions { id, subscriptions });
                }
                Ok(WsMessageV2::Ok { id })
            }
            WsMsgType::ListSubscriptions => {
                let subs = if msg.is_some() {
                    let parsed: WsListSubscriptions = parse_msg(&msg)?;
                    parsed.subscriptions
                } else {
                    subscriptions.unwrap_or_default()
                };
                Ok(WsMessageV2::ListSubscriptions {
                    id,
                    subscriptions: subs,
                })
            }
            WsMsgType::Error => {
                let error = if msg.is_some() {
                    parse_msg(&msg)?
                } else {
                    WsError {
                        code: None,
                        message: None,
                    }
                };
                Ok(WsMessageV2::Error { id, error })
            }
            WsMsgType::Ticker => Ok(WsMessageV2::Data(WsDataMessageV2::Ticker {
                sid,
                seq,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::Trade => Ok(WsMessageV2::Data(WsDataMessageV2::Trade {
                sid,
                seq,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::OrderbookSnapshot => {
                Ok(WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot {
                    sid,
                    seq,
                    msg: parse_msg(&msg)?,
                }))
            }
            WsMsgType::OrderbookDelta => Ok(WsMessageV2::Data(WsDataMessageV2::OrderbookDelta {
                sid,
                seq,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::Fill => Ok(WsMessageV2::Data(WsDataMessageV2::Fill {
                sid,
                seq,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::MarketPosition => Ok(WsMessageV2::Data(WsDataMessageV2::MarketPosition {
                sid,
                seq,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::MarketLifecycleV2 => {
                Ok(WsMessageV2::Data(WsDataMessageV2::MarketLifecycleV2 {
                    sid,
                    seq,
                    msg: parse_msg(&msg)?,
                }))
            }
            WsMsgType::MultivariateMarketLifecycle => Ok(WsMessageV2::Data(
                WsDataMessageV2::MultivariateMarketLifecycle {
                    sid,
                    seq,
                    msg: parse_msg(&msg)?,
                },
            )),
            WsMsgType::EventLifecycle => Ok(WsMessageV2::Data(WsDataMessageV2::EventLifecycle {
                sid,
                seq,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::Multivariate | WsMsgType::MultivariateLookup => {
                Ok(WsMessageV2::Data(WsDataMessageV2::Multivariate {
                    sid,
                    seq,
                    msg: parse_msg(&msg)?,
                }))
            }
            WsMsgType::RfqCreated => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                msg: WsCommunications::RfqCreated(parse_msg(&msg)?),
            })),
            WsMsgType::RfqDeleted => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                msg: WsCommunications::RfqDeleted(parse_msg(&msg)?),
            })),
            WsMsgType::QuoteCreated => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                msg: WsCommunications::QuoteCreated(parse_msg(&msg)?),
            })),
            WsMsgType::QuoteAccepted => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                msg: WsCommunications::QuoteAccepted(parse_msg(&msg)?),
            })),
            WsMsgType::QuoteExecuted => Ok(WsMessageV2::Data(WsDataMessageV2::Communications {
                sid,
                seq,
                msg: WsCommunications::QuoteExecuted(parse_msg(&msg)?),
            })),
            WsMsgType::OrderGroupUpdates => {
                Ok(WsMessageV2::Data(WsDataMessageV2::OrderGroupUpdates {
                    sid,
                    seq,
                    msg: parse_msg(&msg)?,
                }))
            }
            WsMsgType::UserOrder => Ok(WsMessageV2::Data(WsDataMessageV2::UserOrder {
                sid,
                seq,
                msg: parse_msg(&msg)?,
            })),
            WsMsgType::Communications => Ok(WsMessageV2::Unknown {
                msg_type: WsMsgType::Communications,
                raw: msg,
            }),
            other => Ok(WsMessageV2::Unknown {
                msg_type: other,
                raw: msg,
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsEnvelopeRef<'a> {
    pub id: Option<u64>,
    #[serde(rename = "type")]
    pub msg_type: WsMsgType,
    pub sid: Option<u64>,
    pub seq: Option<u64>,
    #[serde(borrow)]
    pub msg: Option<&'a RawValue>,
    #[serde(default, borrow)]
    pub subscriptions: Option<Vec<WsSubscriptionInfoRef<'a>>>,
}

fn parse_borrowed_msg<'a, T: Deserialize<'a>>(
    msg: Option<&'a RawValue>,
) -> Result<T, serde_json::Error> {
    let raw = msg.ok_or_else(|| serde_json::Error::custom("missing msg"))?;
    serde_json::from_str(raw.get())
}

impl<'a> WsEnvelopeRef<'a> {
    pub fn msg_raw(&self) -> Option<&str> {
        self.msg.map(|raw| raw.get())
    }

    pub fn into_message(self) -> Result<WsMessageRef<'a>, KalshiError> {
        #[derive(Deserialize)]
        struct SubscribedMsg {
            #[allow(dead_code)]
            channel: Option<WsChannelV2>,
            sid: Option<u64>,
        }

        let WsEnvelopeRef {
            id,
            msg_type,
            sid,
            seq,
            msg,
            subscriptions,
        } = self;

        match msg_type {
            WsMsgType::Subscribed => {
                let sid = sid.or_else(|| {
                    parse_borrowed_msg::<SubscribedMsg>(msg)
                        .ok()
                        .and_then(|value| value.sid)
                });
                Ok(WsMessageRef::Subscribed { id, sid })
            }
            WsMsgType::Unsubscribed => Ok(WsMessageRef::Unsubscribed { id, sid }),
            WsMsgType::Ok => {
                if msg.is_some()
                    && let Ok(subscriptions) =
                        parse_borrowed_msg::<Vec<WsSubscriptionInfoRef<'a>>>(msg)
                {
                    return Ok(WsMessageRef::ListSubscriptions { id, subscriptions });
                }
                Ok(WsMessageRef::Ok { id })
            }
            WsMsgType::ListSubscriptions => {
                let subs = if msg.is_some() {
                    let parsed: WsListSubscriptionsRef<'a> = parse_borrowed_msg(msg)?;
                    parsed.subscriptions
                } else {
                    subscriptions.unwrap_or_default()
                };
                Ok(WsMessageRef::ListSubscriptions {
                    id,
                    subscriptions: subs,
                })
            }
            WsMsgType::Error => {
                let error = if msg.is_some() {
                    parse_borrowed_msg::<WsErrorRef<'a>>(msg)?
                } else {
                    WsErrorRef {
                        code: None,
                        message: None,
                    }
                };
                Ok(WsMessageRef::Error { id, error })
            }
            WsMsgType::Ticker => Ok(WsMessageRef::Data(WsDataMessageRef::Ticker {
                sid,
                seq,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::Trade => Ok(WsMessageRef::Data(WsDataMessageRef::Trade {
                sid,
                seq,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::OrderbookSnapshot => {
                Ok(WsMessageRef::Data(WsDataMessageRef::OrderbookSnapshot {
                    sid,
                    seq,
                    msg: parse_borrowed_msg(msg)?,
                }))
            }
            WsMsgType::OrderbookDelta => Ok(WsMessageRef::Data(WsDataMessageRef::OrderbookDelta {
                sid,
                seq,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::Fill => Ok(WsMessageRef::Data(WsDataMessageRef::Fill {
                sid,
                seq,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::MarketPosition => Ok(WsMessageRef::Data(WsDataMessageRef::MarketPosition {
                sid,
                seq,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::MarketLifecycleV2 => {
                Ok(WsMessageRef::Data(WsDataMessageRef::MarketLifecycleV2 {
                    sid,
                    seq,
                    msg: parse_borrowed_msg(msg)?,
                }))
            }
            WsMsgType::MultivariateMarketLifecycle => Ok(WsMessageRef::Data(
                WsDataMessageRef::MultivariateMarketLifecycle {
                    sid,
                    seq,
                    msg: parse_borrowed_msg(msg)?,
                },
            )),
            WsMsgType::EventLifecycle => Ok(WsMessageRef::Data(WsDataMessageRef::EventLifecycle {
                sid,
                seq,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::Multivariate | WsMsgType::MultivariateLookup => {
                Ok(WsMessageRef::Data(WsDataMessageRef::Multivariate {
                    sid,
                    seq,
                    msg: parse_borrowed_msg(msg)?,
                }))
            }
            WsMsgType::RfqCreated => Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                msg: WsCommunicationsRef::RfqCreated(parse_borrowed_msg(msg)?),
            })),
            WsMsgType::RfqDeleted => Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                msg: WsCommunicationsRef::RfqDeleted(parse_borrowed_msg(msg)?),
            })),
            WsMsgType::QuoteCreated => Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                msg: WsCommunicationsRef::QuoteCreated(parse_borrowed_msg(msg)?),
            })),
            WsMsgType::QuoteAccepted => Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                msg: WsCommunicationsRef::QuoteAccepted(parse_borrowed_msg(msg)?),
            })),
            WsMsgType::QuoteExecuted => Ok(WsMessageRef::Data(WsDataMessageRef::Communications {
                sid,
                seq,
                msg: WsCommunicationsRef::QuoteExecuted(parse_borrowed_msg(msg)?),
            })),
            WsMsgType::OrderGroupUpdates => {
                Ok(WsMessageRef::Data(WsDataMessageRef::OrderGroupUpdates {
                    sid,
                    seq,
                    msg: parse_borrowed_msg(msg)?,
                }))
            }
            WsMsgType::UserOrder => Ok(WsMessageRef::Data(WsDataMessageRef::UserOrder {
                sid,
                seq,
                msg: parse_borrowed_msg(msg)?,
            })),
            WsMsgType::Communications => Ok(WsMessageRef::Unknown {
                msg_type: WsMsgType::Communications,
                raw: msg,
            }),
            other => Ok(WsMessageRef::Unknown {
                msg_type: other,
                raw: msg,
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsListSubscriptions {
    #[serde(default)]
    pub subscriptions: Vec<WsSubscriptionInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsError {
    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum WsMessageV2 {
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
    },
    Unsubscribed {
        id: Option<u64>,
        sid: Option<u64>,
    },
    ListSubscriptions {
        id: Option<u64>,
        subscriptions: Vec<WsSubscriptionInfo>,
    },
    Ok {
        id: Option<u64>,
    },
    Error {
        id: Option<u64>,
        error: WsError,
    },
    Data(WsDataMessageV2),
    Unknown {
        msg_type: WsMsgType,
        raw: Option<Box<RawValue>>,
    },
}

#[derive(Debug, Clone)]
pub enum WsDataMessageV2 {
    Ticker {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsTicker,
    },
    Trade {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsTrade,
    },
    OrderbookSnapshot {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderbookSnapshot,
    },
    OrderbookDelta {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderbookDelta,
    },
    Fill {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsFill,
    },
    MarketPosition {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketPosition,
    },
    MarketLifecycleV2 {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketLifecycleV2,
    },
    MultivariateMarketLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketLifecycleV2,
    },
    EventLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsEventLifecycle,
    },
    Multivariate {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMultivariate,
    },
    Communications {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsCommunications,
    },
    OrderGroupUpdates {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderGroupUpdate,
    },
    UserOrder {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsUserOrder,
    },
}

#[derive(Debug, Clone)]
pub enum WsDataMessageRef<'a> {
    Ticker {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsTickerRef<'a>,
    },
    Trade {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsTradeRef<'a>,
    },
    OrderbookSnapshot {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderbookSnapshotRef<'a>,
    },
    OrderbookDelta {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderbookDeltaRef<'a>,
    },
    Fill {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsFillRef<'a>,
    },
    MarketPosition {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketPositionRef<'a>,
    },
    MarketLifecycleV2 {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketLifecycleV2Ref<'a>,
    },
    MultivariateMarketLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketLifecycleV2Ref<'a>,
    },
    EventLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsEventLifecycleRef<'a>,
    },
    Multivariate {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMultivariateRef<'a>,
    },
    Communications {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsCommunicationsRef<'a>,
    },
    OrderGroupUpdates {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderGroupUpdateRef<'a>,
    },
    UserOrder {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsUserOrder,
    },
}

impl<'a> WsDataMessageRef<'a> {
    pub fn into_owned(self) -> WsDataMessageV2 {
        match self {
            WsDataMessageRef::Ticker { sid, seq, msg } => WsDataMessageV2::Ticker {
                sid,
                seq,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::Trade { sid, seq, msg } => WsDataMessageV2::Trade {
                sid,
                seq,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::OrderbookSnapshot { sid, seq, msg } => {
                WsDataMessageV2::OrderbookSnapshot {
                    sid,
                    seq,
                    msg: msg.into_owned(),
                }
            }
            WsDataMessageRef::OrderbookDelta { sid, seq, msg } => WsDataMessageV2::OrderbookDelta {
                sid,
                seq,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::Fill { sid, seq, msg } => WsDataMessageV2::Fill {
                sid,
                seq,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::MarketPosition { sid, seq, msg } => WsDataMessageV2::MarketPosition {
                sid,
                seq,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::MarketLifecycleV2 { sid, seq, msg } => {
                WsDataMessageV2::MarketLifecycleV2 {
                    sid,
                    seq,
                    msg: msg.into_owned(),
                }
            }
            WsDataMessageRef::MultivariateMarketLifecycle { sid, seq, msg } => {
                WsDataMessageV2::MultivariateMarketLifecycle {
                    sid,
                    seq,
                    msg: msg.into_owned(),
                }
            }
            WsDataMessageRef::EventLifecycle { sid, seq, msg } => WsDataMessageV2::EventLifecycle {
                sid,
                seq,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::Multivariate { sid, seq, msg } => WsDataMessageV2::Multivariate {
                sid,
                seq,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::Communications { sid, seq, msg } => WsDataMessageV2::Communications {
                sid,
                seq,
                msg: msg.into_owned(),
            },
            WsDataMessageRef::OrderGroupUpdates { sid, seq, msg } => {
                WsDataMessageV2::OrderGroupUpdates {
                    sid,
                    seq,
                    msg: msg.into_owned(),
                }
            }
            WsDataMessageRef::UserOrder { sid, seq, msg } => {
                WsDataMessageV2::UserOrder { sid, seq, msg }
            }
        }
    }
}

/// Borrowed WS message view.
///
/// Note: A smaller, purpose-built struct would be faster, but the library
/// prioritizes feature completeness across all message types.
#[derive(Debug, Clone)]
pub enum WsMessageRef<'a> {
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
    },
    Unsubscribed {
        id: Option<u64>,
        sid: Option<u64>,
    },
    ListSubscriptions {
        id: Option<u64>,
        subscriptions: Vec<WsSubscriptionInfoRef<'a>>,
    },
    Ok {
        id: Option<u64>,
    },
    Error {
        id: Option<u64>,
        error: WsErrorRef<'a>,
    },
    Data(WsDataMessageRef<'a>),
    Unknown {
        msg_type: WsMsgType,
        raw: Option<&'a RawValue>,
    },
}

impl<'a> WsMessageRef<'a> {
    pub fn into_owned(self) -> Result<WsMessageV2, KalshiError> {
        let owned = match self {
            WsMessageRef::Subscribed { id, sid } => WsMessageV2::Subscribed { id, sid },
            WsMessageRef::Unsubscribed { id, sid } => WsMessageV2::Unsubscribed { id, sid },
            WsMessageRef::ListSubscriptions { id, subscriptions } => {
                WsMessageV2::ListSubscriptions {
                    id,
                    subscriptions: subscriptions
                        .into_iter()
                        .map(WsSubscriptionInfoRef::into_owned)
                        .collect(),
                }
            }
            WsMessageRef::Ok { id } => WsMessageV2::Ok { id },
            WsMessageRef::Error { id, error } => WsMessageV2::Error {
                id,
                error: error.into_owned(),
            },
            WsMessageRef::Data(data) => WsMessageV2::Data(data.into_owned()),
            WsMessageRef::Unknown { msg_type, raw } => {
                let raw_owned = match raw {
                    Some(value) => Some(serde_json::from_str::<Box<RawValue>>(value.get())?),
                    None => None,
                };
                WsMessageV2::Unknown {
                    msg_type,
                    raw: raw_owned,
                }
            }
        };
        Ok(owned)
    }
}

#[derive(Debug, Clone)]
pub struct WsRawEvent {
    bytes: Bytes,
}

impl WsRawEvent {
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    pub fn as_str(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes).ok()
    }

    pub fn parse_owned(&self) -> Result<WsMessageV2, KalshiError> {
        WsMessageV2::from_bytes(&self.bytes)
    }

    pub fn parse_borrowed(&self) -> Result<WsMessageRef<'_>, KalshiError> {
        WsMessageRef::from_bytes(&self.bytes)
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct WsSubscribeCmd {
    pub id: u64,
    pub cmd: &'static str, // "subscribe"
    pub params: WsSubscriptionParamsV2,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct WsUnsubscribeCmd {
    pub id: u64,
    pub cmd: &'static str,
    pub params: WsUnsubscribeParamsV2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsUnsubscribeParamsV2 {
    pub sids: Vec<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct WsListSubscriptionsCmd {
    pub id: u64,
    pub cmd: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct WsUpdateSubscriptionCmd {
    pub id: u64,
    pub cmd: &'static str,
    pub params: WsUpdateSubscriptionParamsV2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsUpdateSubscriptionParamsV2 {
    pub action: WsUpdateAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_initial_snapshot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_ticker_ack: Option<bool>,
}

impl WsUpdateSubscriptionParamsV2 {
    pub fn target_sid(&self) -> Option<u64> {
        self.sid.or_else(|| {
            self.sids
                .as_ref()
                .and_then(|values| values.first().copied())
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsUpdateAction {
    AddMarkets,
    DeleteMarkets,
}

pub(crate) fn validate_update(params: &WsUpdateSubscriptionParamsV2) -> Result<(), KalshiError> {
    let has_sid = params.sid.is_some();
    let has_sids = params.sids.is_some();
    if has_sid == has_sids {
        return Err(KalshiError::InvalidParams(
            "update_subscription: provide exactly one of sid or sids".to_string(),
        ));
    }
    if let Some(sids) = &params.sids
        && sids.len() != 1
    {
        return Err(KalshiError::InvalidParams(
            "update_subscription: sids must contain exactly one sid".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn validate_subscription(params: &WsSubscriptionParamsV2) -> Result<(), KalshiError> {
    if params.channels.is_empty() {
        return Err(KalshiError::InvalidParams(
            "subscribe: at least one channel is required".to_string(),
        ));
    }

    let has_market_ticker = params.market_ticker.is_some();
    let has_market_tickers = params
        .market_tickers
        .as_ref()
        .map(|v| !v.is_empty())
        .unwrap_or(false);
    let has_market_id = params.market_id.is_some();
    let has_market_ids = params
        .market_ids
        .as_ref()
        .map(|v| !v.is_empty())
        .unwrap_or(false);
    let has_any_market_tickers = has_market_ticker || has_market_tickers;
    let has_any_market_ids = has_market_id || has_market_ids;

    if has_market_ticker && has_market_tickers {
        return Err(KalshiError::InvalidParams(
            "subscribe: provide at most one of market_ticker or market_tickers".to_string(),
        ));
    }
    if has_market_id && has_market_ids {
        return Err(KalshiError::InvalidParams(
            "subscribe: provide at most one of market_id or market_ids".to_string(),
        ));
    }
    if has_any_market_tickers && has_any_market_ids {
        return Err(KalshiError::InvalidParams(
            "subscribe: market_ticker(s) and market_id(s) are mutually exclusive".to_string(),
        ));
    }

    let has_orderbook_delta = params
        .channels
        .iter()
        .any(|c| matches!(c, WsChannelV2::OrderbookDelta));
    let has_market_positions = params
        .channels
        .iter()
        .any(|c| matches!(c, WsChannelV2::MarketPositions));
    let has_communications = params
        .channels
        .iter()
        .any(|c| matches!(c, WsChannelV2::Communications));

    if has_orderbook_delta && !has_any_market_tickers {
        return Err(KalshiError::InvalidParams(
            "subscribe: orderbook_delta requires market_ticker or market_tickers".to_string(),
        ));
    }

    if has_orderbook_delta && has_any_market_ids {
        return Err(KalshiError::InvalidParams(
            "subscribe: orderbook_delta does not support market_id or market_ids".to_string(),
        ));
    }

    if params.send_initial_snapshot.is_some() && !has_orderbook_delta {
        return Err(KalshiError::InvalidParams(
            "subscribe: send_initial_snapshot only allowed for orderbook_delta".to_string(),
        ));
    }

    if has_any_market_ids && has_market_positions {
        return Err(KalshiError::InvalidParams(
            "subscribe: market_positions only supports market_tickers".to_string(),
        ));
    }

    if params.shard_key.is_some() && params.shard_factor.is_none() {
        return Err(KalshiError::InvalidParams(
            "subscribe: shard_factor is required when shard_key is set".to_string(),
        ));
    }

    if (params.shard_factor.is_some() || params.shard_key.is_some()) && !has_communications {
        return Err(KalshiError::InvalidParams(
            "subscribe: shard_factor/shard_key only allowed for communications".to_string(),
        ));
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum WsWireMessage {
    #[serde(rename = "subscribed")]
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
        #[serde(default)]
        msg: Option<WsSubscribedMsg>,
    },
    #[serde(rename = "unsubscribed")]
    Unsubscribed { id: Option<u64>, sid: Option<u64> },
    #[serde(rename = "ok")]
    Ok {
        id: Option<u64>,
        #[serde(default)]
        msg: Option<Value>,
    },
    #[serde(rename = "list_subscriptions")]
    ListSubscriptions {
        id: Option<u64>,
        #[serde(default)]
        subscriptions: Vec<WsSubscriptionInfo>,
        #[serde(default)]
        msg: Option<WsListSubscriptions>,
    },
    #[serde(rename = "error")]
    Error {
        id: Option<u64>,
        #[serde(default)]
        msg: Option<WsError>,
    },
    #[serde(rename = "ticker")]
    Ticker {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsTicker,
    },
    #[serde(rename = "trade")]
    Trade {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsTrade,
    },
    #[serde(rename = "orderbook_snapshot")]
    OrderbookSnapshot {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderbookSnapshot,
    },
    #[serde(rename = "orderbook_delta")]
    OrderbookDelta {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderbookDelta,
    },
    #[serde(rename = "fill")]
    Fill {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsFill,
    },
    #[serde(rename = "market_position", alias = "market_positions")]
    MarketPosition {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketPosition,
    },
    #[serde(rename = "market_lifecycle_v2")]
    MarketLifecycleV2 {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketLifecycleV2,
    },
    #[serde(rename = "multivariate_market_lifecycle")]
    MultivariateMarketLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMarketLifecycleV2,
    },
    #[serde(rename = "event_lifecycle", alias = "event_lifecycle_v2")]
    EventLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsEventLifecycle,
    },
    #[serde(rename = "multivariate")]
    Multivariate {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMultivariate,
    },
    #[serde(rename = "multivariate_lookup")]
    MultivariateLookup {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsMultivariate,
    },
    #[serde(rename = "rfq_created")]
    RfqCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsRfqCreated,
    },
    #[serde(rename = "rfq_deleted")]
    RfqDeleted {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsRfqDeleted,
    },
    #[serde(rename = "quote_created")]
    QuoteCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsQuoteCreated,
    },
    #[serde(rename = "quote_accepted")]
    QuoteAccepted {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsQuoteAccepted,
    },
    #[serde(rename = "quote_executed")]
    QuoteExecuted {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsQuoteExecuted,
    },
    #[serde(rename = "order_group_updates")]
    OrderGroupUpdates {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsOrderGroupUpdate,
    },
    #[serde(rename = "user_order")]
    UserOrder {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsUserOrder,
    },
}

#[derive(Debug, Deserialize)]
struct WsSubscribedMsg {
    #[allow(dead_code)]
    channel: Option<WsChannelV2>,
    sid: Option<u64>,
}

impl WsWireMessage {
    fn into_message(self) -> WsMessageV2 {
        match self {
            WsWireMessage::Subscribed { id, sid, msg } => WsMessageV2::Subscribed {
                id,
                sid: sid.or_else(|| msg.and_then(|value| value.sid)),
            },
            WsWireMessage::Unsubscribed { id, sid } => WsMessageV2::Unsubscribed { id, sid },
            WsWireMessage::Ok { id, msg } => {
                if let Some(msg) = msg
                    && let Ok(subscriptions) =
                        serde_json::from_value::<Vec<WsSubscriptionInfo>>(msg)
                {
                    return WsMessageV2::ListSubscriptions { id, subscriptions };
                }
                WsMessageV2::Ok { id }
            }
            WsWireMessage::ListSubscriptions {
                id,
                subscriptions,
                msg,
            } => {
                let subs = msg
                    .map(|value| value.subscriptions)
                    .unwrap_or(subscriptions);
                WsMessageV2::ListSubscriptions {
                    id,
                    subscriptions: subs,
                }
            }
            WsWireMessage::Error { id, msg } => WsMessageV2::Error {
                id,
                error: msg.unwrap_or(WsError {
                    code: None,
                    message: None,
                }),
            },
            WsWireMessage::Ticker { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Ticker { sid, seq, msg })
            }
            WsWireMessage::Trade { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Trade { sid, seq, msg })
            }
            WsWireMessage::OrderbookSnapshot { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot { sid, seq, msg })
            }
            WsWireMessage::OrderbookDelta { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::OrderbookDelta { sid, seq, msg })
            }
            WsWireMessage::Fill { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Fill { sid, seq, msg })
            }
            WsWireMessage::MarketPosition { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::MarketPosition { sid, seq, msg })
            }
            WsWireMessage::MarketLifecycleV2 { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::MarketLifecycleV2 { sid, seq, msg })
            }
            WsWireMessage::MultivariateMarketLifecycle { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::MultivariateMarketLifecycle { sid, seq, msg })
            }
            WsWireMessage::EventLifecycle { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::EventLifecycle { sid, seq, msg })
            }
            WsWireMessage::Multivariate { sid, seq, msg }
            | WsWireMessage::MultivariateLookup { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Multivariate { sid, seq, msg })
            }
            WsWireMessage::RfqCreated { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    msg: WsCommunications::RfqCreated(msg),
                })
            }
            WsWireMessage::RfqDeleted { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    msg: WsCommunications::RfqDeleted(msg),
                })
            }
            WsWireMessage::QuoteCreated { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    msg: WsCommunications::QuoteCreated(msg),
                })
            }
            WsWireMessage::QuoteAccepted { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    msg: WsCommunications::QuoteAccepted(msg),
                })
            }
            WsWireMessage::QuoteExecuted { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::Communications {
                    sid,
                    seq,
                    msg: WsCommunications::QuoteExecuted(msg),
                })
            }
            WsWireMessage::OrderGroupUpdates { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::OrderGroupUpdates { sid, seq, msg })
            }
            WsWireMessage::UserOrder { sid, seq, msg } => {
                WsMessageV2::Data(WsDataMessageV2::UserOrder { sid, seq, msg })
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum WsWireMessageRef<'a> {
    #[serde(rename = "subscribed")]
    Subscribed {
        id: Option<u64>,
        sid: Option<u64>,
        #[serde(default)]
        msg: Option<WsSubscribedMsgRef>,
    },
    #[serde(rename = "unsubscribed")]
    Unsubscribed { id: Option<u64>, sid: Option<u64> },
    #[serde(rename = "ok")]
    Ok {
        id: Option<u64>,
        #[serde(default, borrow)]
        msg: Option<&'a RawValue>,
    },
    #[serde(rename = "list_subscriptions")]
    ListSubscriptions {
        id: Option<u64>,
        #[serde(default, borrow)]
        subscriptions: Vec<WsSubscriptionInfoRef<'a>>,
        #[serde(default, borrow)]
        msg: Option<WsListSubscriptionsRef<'a>>,
    },
    #[serde(rename = "error")]
    Error {
        id: Option<u64>,
        #[serde(default, borrow)]
        msg: Option<WsErrorRef<'a>>,
    },
    #[serde(rename = "ticker")]
    Ticker {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsTickerRef<'a>,
    },
    #[serde(rename = "trade")]
    Trade {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsTradeRef<'a>,
    },
    #[serde(rename = "orderbook_snapshot")]
    OrderbookSnapshot {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsOrderbookSnapshotRef<'a>,
    },
    #[serde(rename = "orderbook_delta")]
    OrderbookDelta {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsOrderbookDeltaRef<'a>,
    },
    #[serde(rename = "fill")]
    Fill {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsFillRef<'a>,
    },
    #[serde(rename = "market_position", alias = "market_positions")]
    MarketPosition {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsMarketPositionRef<'a>,
    },
    #[serde(rename = "market_lifecycle_v2")]
    MarketLifecycleV2 {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsMarketLifecycleV2Ref<'a>,
    },
    #[serde(rename = "multivariate_market_lifecycle")]
    MultivariateMarketLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsMarketLifecycleV2Ref<'a>,
    },
    #[serde(rename = "event_lifecycle", alias = "event_lifecycle_v2")]
    EventLifecycle {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsEventLifecycleRef<'a>,
    },
    #[serde(rename = "multivariate")]
    Multivariate {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsMultivariateRef<'a>,
    },
    #[serde(rename = "multivariate_lookup")]
    MultivariateLookup {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsMultivariateRef<'a>,
    },
    #[serde(rename = "rfq_created")]
    RfqCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsRfqCreatedRef<'a>,
    },
    #[serde(rename = "rfq_deleted")]
    RfqDeleted {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsRfqDeletedRef<'a>,
    },
    #[serde(rename = "quote_created")]
    QuoteCreated {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsQuoteCreatedRef<'a>,
    },
    #[serde(rename = "quote_accepted")]
    QuoteAccepted {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsQuoteAcceptedRef<'a>,
    },
    #[serde(rename = "quote_executed")]
    QuoteExecuted {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsQuoteExecutedRef<'a>,
    },
    #[serde(rename = "order_group_updates")]
    OrderGroupUpdates {
        sid: Option<u64>,
        seq: Option<u64>,
        #[serde(borrow)]
        msg: WsOrderGroupUpdateRef<'a>,
    },
    #[serde(rename = "user_order")]
    UserOrder {
        sid: Option<u64>,
        seq: Option<u64>,
        msg: WsUserOrder,
    },
}

#[derive(Debug, Deserialize)]
struct WsSubscribedMsgRef {
    #[allow(dead_code)]
    channel: Option<WsChannelV2>,
    #[serde(default)]
    sid: Option<u64>,
}

impl<'a> WsWireMessageRef<'a> {
    fn into_message(self) -> WsMessageRef<'a> {
        match self {
            WsWireMessageRef::Subscribed { id, sid, msg } => WsMessageRef::Subscribed {
                id,
                sid: sid.or_else(|| msg.and_then(|value| value.sid)),
            },
            WsWireMessageRef::Unsubscribed { id, sid } => WsMessageRef::Unsubscribed { id, sid },
            WsWireMessageRef::Ok { id, msg } => {
                if let Some(raw) = msg
                    && let Ok(subscriptions) =
                        serde_json::from_str::<Vec<WsSubscriptionInfoRef<'a>>>(raw.get())
                {
                    return WsMessageRef::ListSubscriptions { id, subscriptions };
                }
                WsMessageRef::Ok { id }
            }
            WsWireMessageRef::ListSubscriptions {
                id,
                subscriptions,
                msg,
            } => {
                let subs = msg
                    .map(|value| value.subscriptions)
                    .unwrap_or(subscriptions);
                WsMessageRef::ListSubscriptions {
                    id,
                    subscriptions: subs,
                }
            }
            WsWireMessageRef::Error { id, msg } => WsMessageRef::Error {
                id,
                error: msg.unwrap_or(WsErrorRef {
                    code: None,
                    message: None,
                }),
            },
            WsWireMessageRef::Ticker { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Ticker { sid, seq, msg })
            }
            WsWireMessageRef::Trade { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Trade { sid, seq, msg })
            }
            WsWireMessageRef::OrderbookSnapshot { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::OrderbookSnapshot { sid, seq, msg })
            }
            WsWireMessageRef::OrderbookDelta { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::OrderbookDelta { sid, seq, msg })
            }
            WsWireMessageRef::Fill { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Fill { sid, seq, msg })
            }
            WsWireMessageRef::MarketPosition { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::MarketPosition { sid, seq, msg })
            }
            WsWireMessageRef::MarketLifecycleV2 { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::MarketLifecycleV2 { sid, seq, msg })
            }
            WsWireMessageRef::MultivariateMarketLifecycle { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::MultivariateMarketLifecycle { sid, seq, msg })
            }
            WsWireMessageRef::EventLifecycle { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::EventLifecycle { sid, seq, msg })
            }
            WsWireMessageRef::Multivariate { sid, seq, msg }
            | WsWireMessageRef::MultivariateLookup { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Multivariate { sid, seq, msg })
            }
            WsWireMessageRef::RfqCreated { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    msg: WsCommunicationsRef::RfqCreated(msg),
                })
            }
            WsWireMessageRef::RfqDeleted { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    msg: WsCommunicationsRef::RfqDeleted(msg),
                })
            }
            WsWireMessageRef::QuoteCreated { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    msg: WsCommunicationsRef::QuoteCreated(msg),
                })
            }
            WsWireMessageRef::QuoteAccepted { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    msg: WsCommunicationsRef::QuoteAccepted(msg),
                })
            }
            WsWireMessageRef::QuoteExecuted { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::Communications {
                    sid,
                    seq,
                    msg: WsCommunicationsRef::QuoteExecuted(msg),
                })
            }
            WsWireMessageRef::OrderGroupUpdates { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::OrderGroupUpdates { sid, seq, msg })
            }
            WsWireMessageRef::UserOrder { sid, seq, msg } => {
                WsMessageRef::Data(WsDataMessageRef::UserOrder { sid, seq, msg })
            }
        }
    }
}

impl WsMessageV2 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KalshiError> {
        match serde_json::from_slice::<WsWireMessage>(bytes) {
            Ok(wire) => Ok(wire.into_message()),
            Err(first_err) => match serde_json::from_slice::<WsEnvelope>(bytes) {
                Ok(env) => env.into_message(),
                Err(second_err) => Err(KalshiError::parse_reason(
                    "websocket message",
                    bytes,
                    format!(
                        "failed to parse as WsWireMessage ({first_err}); failed to parse as WsEnvelope ({second_err})"
                    ),
                )),
            },
        }
        .map_err(|err| match err {
            KalshiError::Json(source) => {
                KalshiError::parse_json("websocket message payload", bytes, source)
            }
            other => other,
        })
    }
}

impl<'a> WsMessageRef<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, KalshiError> {
        match serde_json::from_slice::<WsWireMessageRef<'a>>(bytes) {
            Ok(wire) => Ok(wire.into_message()),
            Err(first_err) => match serde_json::from_slice::<WsEnvelopeRef<'a>>(bytes) {
                Ok(env) => env.into_message(),
                Err(second_err) => Err(KalshiError::parse_reason(
                    "websocket borrowed message",
                    bytes,
                    format!(
                        "failed to parse as WsWireMessageRef ({first_err}); failed to parse as WsEnvelopeRef ({second_err})"
                    ),
                )),
            },
        }
        .map_err(|err| match err {
            KalshiError::Json(source) => {
                KalshiError::parse_json("websocket borrowed message payload", bytes, source)
            }
            other => other,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn validate_subscription_requires_market_tickers_for_orderbook_delta() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());

        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["TEST".to_string()]),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_ok());
    }

    #[test]
    fn validate_subscription_send_initial_snapshot_only_for_orderbook_delta() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            send_initial_snapshot: Some(true),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());
    }

    #[test]
    fn validate_subscription_orderbook_delta_rejects_market_ids() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_ids: Some(vec!["mid-1".to_string()]),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());
    }

    #[test]
    fn validate_subscription_rejects_market_positions_with_market_ids() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::MarketPositions],
            market_ids: Some(vec!["mid-1".to_string()]),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());
    }

    #[test]
    fn validate_subscription_shard_fields_require_communications() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            shard_factor: Some(2),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());

        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Communications],
            shard_factor: Some(2),
            shard_key: Some(1),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_ok());
    }

    #[test]
    fn validate_subscription_send_initial_snapshot_with_orderbook_delta_ok() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["TEST".to_string()]),
            send_initial_snapshot: Some(true),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_ok());
    }

    #[test]
    fn ws_msg_type_deserialize_known() {
        let msg_type: WsMsgType = serde_json::from_str("\"trade\"").unwrap();
        assert!(matches!(msg_type, WsMsgType::Trade));
    }

    #[test]
    fn ws_msg_type_deserialize_unknown() {
        let msg_type: WsMsgType = serde_json::from_str("\"new_type\"").unwrap();
        assert!(matches!(msg_type, WsMsgType::Unknown(value) if value == "new_type"));
    }

    #[test]
    fn ws_envelope_into_message_known_type() {
        let json = r#"{
            "type":"ticker",
            "sid":1,
            "seq":2,
            "msg":{
                "market_ticker":"TEST",
                "market_id":"1",
                "price_dollars":"0.01",
                "yes_bid_dollars":"0.01",
                "yes_ask_dollars":"0.02",
                "yes_bid_size_fp":"1.00",
                "yes_ask_size_fp":"2.00",
                "last_trade_size_fp":"1.00",
                "volume_fp":"0",
                "open_interest_fp":"0",
                "dollar_volume":0,
                "dollar_open_interest":0,
                "ts":0,
                "ts_ms":0,
                "time":"1970-01-01T00:00:00Z"
            }
        }"#;
        let env: WsEnvelope = serde_json::from_str(json).unwrap();
        let msg = env.into_message().unwrap();
        assert!(matches!(
            msg,
            WsMessageV2::Data(WsDataMessageV2::Ticker { .. })
        ));
    }

    #[test]
    fn ws_envelope_into_message_unknown_type() {
        let json = r#"{"type":"mystery","msg":{"foo":1}}"#;
        let env: WsEnvelope = serde_json::from_str(json).unwrap();
        let msg = env.into_message().unwrap();
        match msg {
            WsMessageV2::Unknown {
                msg_type: WsMsgType::Unknown(value),
                raw,
            } => {
                assert_eq!(value, "mystery");
                assert!(raw.is_some());
            }
            _ => panic!("expected unknown message"),
        }
    }

    #[test]
    fn ws_message_from_bytes_known_type() {
        let json = r#"{
            "type":"ticker",
            "sid":1,
            "seq":2,
            "msg":{
                "market_ticker":"TEST",
                "market_id":"1",
                "price_dollars":"0.01",
                "yes_bid_dollars":"0.01",
                "yes_ask_dollars":"0.02",
                "yes_bid_size_fp":"1.00",
                "yes_ask_size_fp":"2.00",
                "last_trade_size_fp":"1.00",
                "volume_fp":"0",
                "open_interest_fp":"0",
                "dollar_volume":0,
                "dollar_open_interest":0,
                "ts":0,
                "ts_ms":0,
                "time":"1970-01-01T00:00:00Z"
            }
        }"#;
        let msg = WsMessageV2::from_bytes(json.as_bytes()).unwrap();
        assert!(matches!(
            msg,
            WsMessageV2::Data(WsDataMessageV2::Ticker { .. })
        ));
    }

    #[test]
    fn ws_message_from_bytes_unknown_type() {
        let json = r#"{"type":"mystery","msg":{"foo":1}}"#;
        let msg = WsMessageV2::from_bytes(json.as_bytes()).unwrap();
        match msg {
            WsMessageV2::Unknown {
                msg_type: WsMsgType::Unknown(value),
                raw,
            } => {
                assert_eq!(value, "mystery");
                assert!(raw.is_some());
            }
            _ => panic!("expected unknown message"),
        }
    }

    #[test]
    fn ws_message_from_bytes_invalid_json_exposes_raw_bytes_and_reason() {
        let raw = br#"{"type":"ticker","msg":{"market_ticker":"TEST"}"#;
        let err = WsMessageV2::from_bytes(raw).expect_err("invalid JSON should fail");
        match err {
            KalshiError::Parse {
                context,
                reason,
                raw: parse_raw,
                ..
            } => {
                assert_eq!(context, "websocket message");
                assert_eq!(parse_raw.as_slice(), raw);
                assert!(reason.contains("failed to parse as WsWireMessage"));
                assert!(reason.contains("failed to parse as WsEnvelope"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn ws_message_from_bytes_payload_parse_error_exposes_raw_bytes_and_reason() {
        let raw = br#"{"type":"ticker","sid":1,"seq":2,"msg":{"market_ticker":"TEST"}}"#;
        let err = WsMessageV2::from_bytes(raw).expect_err("invalid payload should fail");
        assert_eq!(err.parse_context(), Some("websocket message payload"));
        assert_eq!(err.parse_raw_bytes(), Some(&raw[..]));
        let reason = err
            .parse_error_reason()
            .expect("parse errors should include a reason");
        assert!(reason.contains("missing field"));
    }

    #[test]
    fn ws_message_ref_roundtrip_owned() {
        let json = r#"{
            "type":"trade",
            "sid":3,
            "seq":4,
            "msg":{
                "trade_id":"t1",
                "market_ticker":"TST",
                "count_fp":"2",
                "yes_price_dollars":"0.10",
                "no_price_dollars":"0.90",
                "taker_side":"yes",
                "ts":1704067200,
                "ts_ms":1704067200000
            }
        }"#;
        let msg_ref = WsMessageRef::from_bytes(json.as_bytes()).unwrap();
        let msg = msg_ref.into_owned().unwrap();
        assert!(matches!(
            msg,
            WsMessageV2::Data(WsDataMessageV2::Trade { .. })
        ));
    }

    #[test]
    fn ws_raw_event_parse_borrowed() {
        let json = r#"{
            "type":"ticker",
            "sid":9,
            "seq":10,
            "msg":{
                "market_ticker":"TEST",
                "market_id":"1",
                "price_dollars":"0.01",
                "yes_bid_dollars":"0.01",
                "yes_ask_dollars":"0.02",
                "yes_bid_size_fp":"1.00",
                "yes_ask_size_fp":"2.00",
                "last_trade_size_fp":"1.00",
                "volume_fp":"0",
                "open_interest_fp":"0",
                "dollar_volume":0,
                "dollar_open_interest":0,
                "ts":0,
                "ts_ms":0,
                "time":"1970-01-01T00:00:00Z"
            }
        }"#;
        let raw = WsRawEvent::new(Bytes::from(json));
        let msg = raw.parse_borrowed().unwrap();
        assert!(matches!(
            msg,
            WsMessageRef::Data(WsDataMessageRef::Ticker { .. })
        ));
    }

    #[test]
    fn ws_orderbook_delta_side_parse() {
        let json = r#"{
            "market_ticker":"TEST",
            "market_id":"1",
            "price_dollars":"0.01",
            "delta_fp":"1",
            "side":"yes"
        }"#;
        let delta: WsOrderbookDelta = serde_json::from_str(json).unwrap();
        assert!(matches!(delta.side, YesNo::Yes));
    }

    #[test]
    fn ws_fill_side_action_parse() {
        let json = r#"{
            "trade_id":"t",
            "order_id":"o",
            "market_ticker":"T",
            "side":"no",
            "action":"buy",
            "count_fp":"1",
            "yes_price_dollars":"0.01",
            "is_taker":true,
            "fee_cost":"0.00",
            "ts":0,
            "ts_ms":0,
            "post_position_fp":"1.00",
            "purchased_side":"yes"
        }"#;
        let fill: WsFill = serde_json::from_str(json).unwrap();
        assert!(matches!(fill.side, YesNo::No));
        assert!(matches!(fill.action, BuySell::Buy));
    }

    #[test]
    fn validate_update_requires_exactly_one_sid_target() {
        let both = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::AddMarkets,
            sid: Some(1),
            sids: Some(vec![2]),
            market_ticker: Some("TICKER".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
        };
        assert!(validate_update(&both).is_err());

        let multi = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::AddMarkets,
            sid: None,
            sids: Some(vec![1, 2]),
            market_ticker: Some("TICKER".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
        };
        assert!(validate_update(&multi).is_err());

        let valid = WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::DeleteMarkets,
            sid: Some(1),
            sids: None,
            market_ticker: Some("TICKER".to_string()),
            market_tickers: None,
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: None,
        };
        assert!(validate_update(&valid).is_ok());
    }

    #[test]
    fn validate_subscription_enforces_market_target_exclusivity() {
        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            market_ticker: Some("A".to_string()),
            market_tickers: Some(vec!["B".to_string()]),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());

        let params = WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            market_ticker: Some("A".to_string()),
            market_id: Some("uuid".to_string()),
            ..Default::default()
        };
        assert!(validate_subscription(&params).is_err());
    }
}
