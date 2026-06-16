use crate::error::KalshiError;
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Top-level message (discriminated by `type` field via untagged fallthrough)
// ---------------------------------------------------------------------------

/// A single data message received from the margin/perpetuals WebSocket.
///
/// All margin WS messages share the envelope shape:
/// ```json
/// { "type": "<...>", "sid": <int>, "seq": <int>?, "msg": { ... } }
/// ```
///
/// Control messages (`subscribed`/`unsubscribed`) are parsed by
/// [`parse_control_message`](crate::ws::protocol::parse_control_message) in the
/// reader loop and forwarded as [`WsEvent::Message`]. They arrive as
/// `Unknown { msg_type: Some("subscribed"|"unsubscribed") }` here.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum MarginDataMessage {
    /// Must come before OrderbookSnapshot — required fields make it specific.
    Ticker(MarginEnvelope<TickerMsg>),
    OrderbookDelta(MarginEnvelope<OrderbookDeltaMsg>),
    Trade(MarginEnvelope<TradeMsg>),
    Fill(MarginEnvelope<FillMsg>),
    UserOrder(MarginEnvelope<UserOrderMsg>),
    OrderGroupUpdates(MarginEnvelope<OrderGroupUpdatesMsg>),
    /// Deliberately last among known types: `bid`/`ask` are optional so it
    /// would spuriously match narrower messages.
    OrderbookSnapshot(MarginEnvelope<OrderbookSnapshotMsg>),
    /// Catch-all for subscribed/unsubscribed control messages and unknown
    /// future message types.
    Unknown {
        #[serde(rename = "type")]
        msg_type: Option<String>,
    },
}

impl MarginDataMessage {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KalshiError> {
        serde_json::from_slice(bytes).map_err(KalshiError::from)
    }
}

// ---------------------------------------------------------------------------
// Generic envelope
// ---------------------------------------------------------------------------

/// Shared envelope with `sid`, optional `seq`, and a typed `msg` payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MarginEnvelope<T> {
    pub sid: u64,
    #[serde(default)]
    pub seq: Option<u64>,
    pub msg: T,
}

// ---------------------------------------------------------------------------
// orderbook_snapshot  —  full orderbook depth
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrderbookSnapshotMsg {
    pub market_ticker: String,
    #[serde(default)]
    pub bid: Vec<Vec<String>>,
    #[serde(default)]
    pub ask: Vec<Vec<String>>,
}

// ---------------------------------------------------------------------------
// orderbook_delta  —  incremental price-level change
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrderbookDeltaMsg {
    pub market_ticker: String,
    pub price: String,
    pub delta: String,
    pub side: String,
    #[serde(default)]
    pub last_update_reason: Option<String>,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub subaccount: Option<i64>,
    #[serde(default)]
    pub ts_ms: Option<i64>,
}

// ---------------------------------------------------------------------------
// ticker  —  market summary
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TickerPrice {
    pub price: String,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TickerFundingRate {
    pub rate: f64,
    pub next_funding_time_ms: i64,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TickerMsg {
    pub market_ticker: String,
    pub price: String,
    pub bid: String,
    pub ask: String,
    pub bid_size_fp: String,
    pub ask_size_fp: String,
    pub last_trade_size_fp: String,
    pub volume: String,
    pub volume_notional_value_dollars: String,
    pub volume_24h: String,
    pub volume_24h_notional_value_dollars: String,
    pub open_interest: String,
    pub open_interest_notional_value_dollars: String,
    #[serde(default)]
    pub reference_price: Option<TickerPrice>,
    #[serde(default)]
    pub settlement_mark_price: Option<TickerPrice>,
    #[serde(default)]
    pub liquidation_mark_price: Option<TickerPrice>,
    #[serde(default)]
    pub funding_rate: Option<TickerFundingRate>,
    pub ts_ms: i64,
}

// ---------------------------------------------------------------------------
// trade  —  public trade notification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TradeMsg {
    pub trade_id: String,
    pub market_ticker: String,
    pub price: String,
    pub count: String,
    pub taker_side: String,
    pub ts_ms: i64,
}

// ---------------------------------------------------------------------------
// fill  —  private fill notification (requires auth)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FillMsg {
    pub trade_id: String,
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub market_ticker: String,
    pub is_taker: bool,
    pub side: String,
    pub ts_ms: i64,
    pub price: String,
    pub count: String,
    pub fee_cost: String,
    pub post_position: String,
    #[serde(default)]
    pub subaccount: Option<i64>,
}

// ---------------------------------------------------------------------------
// user_order  —  private order update (requires auth)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UserOrderMsg {
    pub order_id: String,
    pub user_id: String,
    pub client_order_id: String,
    pub ticker: String,
    pub side: String,
    pub price: String,
    pub fill_count: String,
    pub remaining_count: String,
    #[serde(default)]
    pub self_trade_prevention_type: Option<String>,
    #[serde(default)]
    pub order_group_id: Option<String>,
    #[serde(default)]
    pub expiration_ts_ms: Option<i64>,
    pub created_ts_ms: i64,
    #[serde(default)]
    pub last_updated_ts_ms: Option<i64>,
    #[serde(default)]
    pub subaccount_number: Option<i64>,
}

// ---------------------------------------------------------------------------
// order_group_updates  —  order-group lifecycle
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrderGroupUpdatesMsg {
    pub event_type: String,
    pub order_group_id: String,
    #[serde(default)]
    pub contracts_limit_fp: Option<String>,
    pub ts_ms: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_orderbook_snapshot() {
        let json = serde_json::json!({
            "type": "orderbook_snapshot",
            "sid": 1,
            "seq": 100,
            "msg": {
                "market_ticker": "KXBTCPERP1",
                "bid": [["50000.00", "1.5"]],
                "ask": [["50100.00", "2.0"]]
            }
        });
        let msg: MarginDataMessage = serde_json::from_value(json).unwrap();
        match msg {
            MarginDataMessage::OrderbookSnapshot(e) => {
                assert_eq!(e.sid, 1);
                assert_eq!(e.seq, Some(100));
                assert_eq!(e.msg.market_ticker, "KXBTCPERP1");
                assert_eq!(e.msg.bid[0], vec!["50000.00", "1.5"]);
            }
            _ => panic!("expected OrderbookSnapshot"),
        }
    }

    #[test]
    fn parse_orderbook_delta() {
        let json = serde_json::json!({
            "type": "orderbook_delta",
            "sid": 1,
            "seq": 101,
            "msg": {
                "market_ticker": "KXBTCPERP1",
                "price": "50000.00",
                "delta": "1.5",
                "side": "bid",
                "ts_ms": 1_700_000_000_000_i64
            }
        });
        let msg: MarginDataMessage = serde_json::from_value(json).unwrap();
        match msg {
            MarginDataMessage::OrderbookDelta(e) => {
                assert_eq!(e.msg.market_ticker, "KXBTCPERP1");
                assert_eq!(e.msg.side, "bid");
                assert_eq!(e.msg.ts_ms, Some(1_700_000_000_000));
            }
            _ => panic!("expected OrderbookDelta"),
        }
    }

    #[test]
    fn parse_ticker() {
        let json = serde_json::json!({
            "type": "ticker",
            "sid": 2,
            "msg": {
                "market_ticker": "KXBTCPERP1",
                "price": "50000.0000",
                "bid": "49900.0000",
                "ask": "50100.0000",
                "bid_size_fp": "10.0",
                "ask_size_fp": "5.0",
                "last_trade_size_fp": "1.0",
                "volume": "1000",
                "volume_notional_value_dollars": "50000000",
                "volume_24h": "500",
                "volume_24h_notional_value_dollars": "25000000",
                "open_interest": "200",
                "open_interest_notional_value_dollars": "10000000",
                "ts_ms": 1_700_000_000_000_i64
            }
        });
        let msg: MarginDataMessage = serde_json::from_value(json).unwrap();
        match msg {
            MarginDataMessage::Ticker(e) => {
                assert_eq!(e.msg.market_ticker, "KXBTCPERP1");
                assert_eq!(e.msg.price, "50000.0000");
            }
            _ => panic!("expected Ticker"),
        }
    }

    #[test]
    fn parse_trade() {
        let json = serde_json::json!({
            "type": "trade",
            "sid": 3,
            "msg": {
                "trade_id": "t-123",
                "market_ticker": "KXBTCPERP1",
                "price": "50000.00",
                "count": "1.0",
                "taker_side": "bid",
                "ts_ms": 1_700_000_000_000_i64
            }
        });
        let msg: MarginDataMessage = serde_json::from_value(json).unwrap();
        match msg {
            MarginDataMessage::Trade(e) => {
                assert_eq!(e.msg.trade_id, "t-123");
                assert_eq!(e.msg.taker_side, "bid");
            }
            _ => panic!("expected Trade"),
        }
    }

    #[test]
    fn parse_fill() {
        let json = serde_json::json!({
            "type": "fill",
            "sid": 4,
            "msg": {
                "trade_id": "t-456",
                "order_id": "o-789",
                "client_order_id": "co-001",
                "market_ticker": "KXBTCPERP1",
                "is_taker": true,
                "side": "ask",
                "ts_ms": 1_700_000_000_000_i64,
                "price": "50000.00",
                "count": "1.0",
                "fee_cost": "0.50",
                "post_position": "5.0",
                "subaccount": 1
            }
        });
        let msg: MarginDataMessage = serde_json::from_value(json).unwrap();
        match msg {
            MarginDataMessage::Fill(e) => {
                assert!(e.msg.is_taker);
                assert_eq!(e.msg.side, "ask");
                assert_eq!(e.msg.subaccount, Some(1));
            }
            _ => panic!("expected Fill"),
        }
    }

    #[test]
    fn parse_user_order() {
        let json = serde_json::json!({
            "type": "user_order",
            "sid": 5,
            "msg": {
                "order_id": "o-789",
                "user_id": "u-001",
                "client_order_id": "co-001",
                "ticker": "KXBTCPERP1",
                "side": "bid",
                "price": "50000.00",
                "fill_count": "0",
                "remaining_count": "1.0",
                "created_ts_ms": 1_700_000_000_000_i64
            }
        });
        let msg: MarginDataMessage = serde_json::from_value(json).unwrap();
        match msg {
            MarginDataMessage::UserOrder(e) => {
                assert_eq!(e.msg.side, "bid");
                assert_eq!(e.msg.fill_count, "0");
            }
            _ => panic!("expected UserOrder"),
        }
    }

    #[test]
    fn parse_order_group_updates() {
        let json = serde_json::json!({
            "type": "order_group_updates",
            "sid": 21,
            "seq": 7,
            "msg": {
                "event_type": "limit_updated",
                "order_group_id": "og_123",
                "contracts_limit_fp": "150.00",
                "ts_ms": 1_700_000_000_000_i64
            }
        });
        let msg: MarginDataMessage = serde_json::from_value(json).unwrap();
        match msg {
            MarginDataMessage::OrderGroupUpdates(e) => {
                assert_eq!(e.msg.event_type, "limit_updated");
                assert_eq!(e.msg.order_group_id, "og_123");
            }
            _ => panic!("expected OrderGroupUpdates"),
        }
    }

    #[test]
    fn parse_subscribed_as_unknown() {
        let json = serde_json::json!({
            "type": "subscribed",
            "id": 1,
            "sid": 42
        });
        let msg: MarginDataMessage = serde_json::from_value(json).unwrap();
        assert_eq!(
            msg,
            MarginDataMessage::Unknown {
                msg_type: Some("subscribed".into())
            }
        );
    }

    #[test]
    fn parse_unsubscribed_as_unknown() {
        let json = serde_json::json!({
            "type": "unsubscribed",
            "sid": 42
        });
        let msg: MarginDataMessage = serde_json::from_value(json).unwrap();
        assert_eq!(
            msg,
            MarginDataMessage::Unknown {
                msg_type: Some("unsubscribed".into())
            }
        );
    }

    #[test]
    fn parse_unknown_type() {
        let json = serde_json::json!({
            "type": "some_future_channel",
            "sid": 1,
            "msg": {"foo": "bar"}
        });
        let msg: MarginDataMessage = serde_json::from_value(json).unwrap();
        assert_eq!(
            msg,
            MarginDataMessage::Unknown {
                msg_type: Some("some_future_channel".into())
            }
        );
    }
}
