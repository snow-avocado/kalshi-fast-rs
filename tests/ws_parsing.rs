//! Unit tests for WebSocket message parsing.

use kalshi_fast::{
    WsCommunications, WsDataMessageV2, WsEnvelope, WsMarketLifecycleEventType, WsMessageV2,
    WsMsgType, WsOrderGroupEventType, WsOrderbookDelta, WsTicker, YesNo,
};
use serde_json::Value;

#[test]
fn ws_envelope_deserializes_with_sid_and_seq() {
    let json = r#"{
        "id": 1,
        "type": "orderbook_snapshot",
        "sid": 42,
        "seq": 100,
        "msg": {"market_ticker": "TEST", "market_id": "abc123"}
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    assert_eq!(env.id, Some(1));
    assert_eq!(env.msg_type, WsMsgType::OrderbookSnapshot);
    assert_eq!(env.sid, Some(42));
    assert_eq!(env.seq, Some(100));
    assert!(env.msg.is_some());
}

#[test]
fn ws_message_subscribed_parses() {
    let json = r#"{
        "id": 5,
        "type": "subscribed",
        "sid": 99
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Subscribed { id, sid } => {
            assert_eq!(id, Some(5));
            assert_eq!(sid, Some(99));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_message_subscribed_parses_sid_from_msg() {
    let json = r#"{
        "id": 6,
        "type": "subscribed",
        "msg": {
            "channel": "ticker",
            "sid": 321
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Subscribed { id, sid } => {
            assert_eq!(id, Some(6));
            assert_eq!(sid, Some(321));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_message_list_subscriptions_parses() {
    let json = r#"{
        "id": 7,
        "type": "list_subscriptions",
        "msg": {
            "subscriptions": [
                {"sid": 1, "channels": ["ticker"], "market_tickers": ["TEST"]}
            ]
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::ListSubscriptions { id, subscriptions } => {
            assert_eq!(id, Some(7));
            assert_eq!(subscriptions.len(), 1);
            assert_eq!(subscriptions[0].sid, 1);
            assert_eq!(subscriptions[0].market_tickers.as_ref().unwrap()[0], "TEST");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_message_list_subscriptions_parses_from_ok_msg_array() {
    let json = r#"{
        "id": 8,
        "type": "ok",
        "msg": [
            {"channel": "ticker", "sid": 11},
            {"channel": "trade", "sid": 12}
        ]
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::ListSubscriptions { id, subscriptions } => {
            assert_eq!(id, Some(8));
            assert_eq!(subscriptions.len(), 2);
            assert_eq!(subscriptions[0].sid, 11);
            assert_eq!(
                subscriptions[0].channel,
                Some(kalshi_fast::WsChannelV2::Ticker)
            );
            assert_eq!(subscriptions[1].sid, 12);
            assert_eq!(
                subscriptions[1].channel,
                Some(kalshi_fast::WsChannelV2::Trade)
            );
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_message_error_parses() {
    let json = r#"{
        "id": 9,
        "type": "error",
        "msg": {"code": 9, "message": "Authentication required"}
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Error { id, error } => {
            assert_eq!(id, Some(9));
            assert_eq!(error.code, Some(9));
            assert_eq!(error.message.as_deref(), Some("Authentication required"));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_ticker_message_parses() {
    let json = r#"{
        "type": "ticker",
        "msg": {
            "market_ticker": "INXD-25JAN10-T17900",
            "market_id": "abc123",
            "price_dollars": "0.55",
            "yes_bid_dollars": "0.54",
            "yes_ask_dollars": "0.56",
            "yes_bid_size_fp": "10.00",
            "yes_ask_size_fp": "12.00",
            "last_trade_size_fp": "2.00",
            "volume_fp": "10000.00",
            "open_interest_fp": "5000.00",
            "dollar_volume": 5500,
            "dollar_open_interest": 2750,
            "ts": 1700000000,
            "ts_ms": 1700000000000,
            "time": "2025-01-10T12:00:00Z"
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::Ticker { msg, .. }) => {
            assert_eq!(msg.market_ticker, "INXD-25JAN10-T17900");
            assert_eq!(msg.price_dollars, "0.55");
            assert_eq!(msg.yes_bid_size_fp, "10.00");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_trade_message_parses() {
    let json = r#"{
        "type": "trade",
        "msg": {
            "trade_id": "trade-1",
            "market_ticker": "MKT-1",
            "yes_price_dollars": "0.55",
            "no_price_dollars": "0.45",
            "count_fp": "10.00",
            "taker_side": "yes",
            "ts": 1700000000,
            "ts_ms": 1700000000000
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::Trade { msg, .. }) => {
            assert_eq!(msg.trade_id, "trade-1");
            assert_eq!(msg.market_ticker, "MKT-1");
            assert_eq!(msg.count_fp, "10.00");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_orderbook_snapshot_deserializes() {
    let json = r#"{
        "type": "orderbook_snapshot",
        "msg": {
            "market_ticker": "INXD-25JAN10-T17900",
            "market_id": "abc123",
            "yes_dollars_fp": [["0.50", "100.00"], ["0.51", "200.00"]],
            "no_dollars_fp": [["0.49", "150.00"]]
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::OrderbookSnapshot { msg, .. }) => {
            assert_eq!(msg.market_ticker, "INXD-25JAN10-T17900");
            assert_eq!(msg.yes_dollars_fp.len(), 2);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_orderbook_delta_deserializes() {
    let json = r#"{
        "type": "orderbook_delta",
        "msg": {
            "market_ticker": "INXD-25JAN10-T17900",
            "market_id": "abc123",
            "price_dollars": "0.55",
            "delta_fp": "50.00",
            "side": "yes",
            "client_order_id": "my-order-1",
            "subaccount": 0,
            "ts": "2025-01-10T12:00:00Z",
            "ts_ms": 1700000000000
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::OrderbookDelta { msg, .. }) => {
            assert_eq!(msg.market_ticker, "INXD-25JAN10-T17900");
            assert_eq!(msg.delta_fp, "50.00");
            assert_eq!(msg.ts_ms, Some(1700000000000));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_fill_deserializes() {
    let json = r#"{
        "type": "fill",
        "msg": {
            "trade_id": "trade-456",
            "order_id": "order-789",
            "client_order_id": "my-order",
            "market_ticker": "INXD-25JAN10-T17900",
            "side": "yes",
            "action": "buy",
            "count_fp": "10.00",
            "yes_price_dollars": "0.55",
            "is_taker": true,
            "fee_cost": "0.05",
            "ts": 1700000000,
            "ts_ms": 1700000000000,
            "post_position_fp": "15.00",
            "purchased_side": "yes",
            "subaccount": 1
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::Fill { msg, .. }) => {
            assert_eq!(msg.trade_id, "trade-456");
            assert_eq!(msg.market_ticker, "INXD-25JAN10-T17900");
            assert_eq!(msg.post_position_fp, "15.00");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_envelope_parse_ticker_raw() {
    let json = r#"{
        "type": "ticker",
        "sid": 1,
        "msg": {
            "market_ticker": "TEST",
            "market_id": "abc",
            "price_dollars": "0.50",
            "yes_bid_dollars": "0.49",
            "yes_ask_dollars": "0.51",
            "yes_bid_size_fp": "2.00",
            "yes_ask_size_fp": "3.00",
            "last_trade_size_fp": "1.00",
            "volume_fp": "1000.00",
            "open_interest_fp": "500.00",
            "dollar_volume": 500,
            "dollar_open_interest": 250,
            "ts": 1700000000,
            "ts_ms": 1700000000000,
            "time": "2025-01-10T12:00:00Z"
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let raw = env.msg_raw().unwrap();
    let msg: WsTicker = serde_json::from_str(raw).unwrap();
    assert_eq!(msg.market_ticker, "TEST");
}

#[test]
fn ws_orderbook_delta_raw() {
    let json = r#"{
        "type": "orderbook_delta",
        "msg": {
            "market_ticker": "TEST",
            "market_id": "abc",
            "price_dollars": "0.50",
            "delta_fp": "10.00",
            "side": "yes",
            "ts_ms": 1700000000000
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let raw = env.msg_raw().unwrap();
    let msg: WsOrderbookDelta = serde_json::from_str(raw).unwrap();
    assert_eq!(msg.delta_fp, "10.00");
}

#[test]
fn ws_market_lifecycle_v2_message_parses() {
    let json = r#"{
        "type": "market_lifecycle_v2",
        "msg": {
            "market_ticker": "MKT-1",
            "event_type": "created",
            "open_ts": 1700000000,
            "close_ts": 1700003600,
            "additional_metadata": {
                "name": "Test market",
                "strike_type": "greater",
                "floor_strike": 123,
                "custom_strike": {"foo": "bar"},
                "unknown_field": "keep"
            }
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::MarketLifecycleV2 { msg, .. }) => {
            assert_eq!(msg.market_ticker, "MKT-1");
            assert_eq!(msg.event_type, Some(WsMarketLifecycleEventType::Created));
            assert_eq!(msg.open_ts, Some(1700000000));
            assert_eq!(msg.close_ts, Some(1700003600));
            let metadata = msg
                .additional_metadata
                .expect("missing additional_metadata");
            let custom = metadata.custom_strike.expect("missing custom_strike");
            assert_eq!(custom.get("foo").map(String::as_str), Some("bar"));
            assert_eq!(
                metadata.extra.get("unknown_field"),
                Some(&Value::String("keep".to_string()))
            );
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_event_lifecycle_message_parses() {
    let json = r#"{
        "type": "event_lifecycle",
        "msg": {
            "event_ticker": "EVT-1",
            "title": "Event title",
            "subtitle": "Event subtitle",
            "collateral_return_type": "standard",
            "series_ticker": "SER-1",
            "additional_metadata": {
                "custom_strike": {"a": "b"},
                "extra_field": 42
            }
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::EventLifecycle { msg, .. }) => {
            assert_eq!(msg.event_ticker, "EVT-1");
            assert_eq!(msg.title.as_deref(), Some("Event title"));
            assert_eq!(msg.subtitle.as_deref(), Some("Event subtitle"));
            assert_eq!(msg.collateral_return_type.as_deref(), Some("standard"));
            assert_eq!(msg.series_ticker.as_deref(), Some("SER-1"));
            let metadata = msg
                .additional_metadata
                .expect("missing additional_metadata");
            let custom = metadata.custom_strike.expect("missing custom_strike");
            assert_eq!(custom.get("a").map(String::as_str), Some("b"));
            assert_eq!(
                metadata.extra.get("extra_field"),
                Some(&Value::Number(42.into()))
            );
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_event_lifecycle_v2_alias_parses() {
    let json = r#"{
        "type": "event_lifecycle_v2",
        "msg": {
            "event_ticker": "EVT-ALIAS"
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::EventLifecycle { msg, .. }) => {
            assert_eq!(msg.event_ticker, "EVT-ALIAS");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_market_positions_message_parses() {
    let json = r#"{
        "type": "market_position",
        "msg": {
            "user_id": "user-1",
            "market_ticker": "MKT-1",
            "position_fp": "1.00",
            "position_cost_dollars": "0.52",
            "realized_pnl_dollars": "0.10",
            "fees_paid_dollars": "0.01",
            "position_fee_cost_dollars": "0.02",
            "volume_fp": "10.00"
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::MarketPosition { msg, .. }) => {
            assert_eq!(msg.market_ticker, "MKT-1");
            assert_eq!(msg.position_fp, "1.00");
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_rfq_created_message_parses() {
    let json = r#"{
        "type": "rfq_created",
        "msg": {
            "id": "rfq_123",
            "creator_id": "",
            "market_ticker": "FED-23DEC-T3.00",
            "created_ts": "2024-12-01T10:00:00Z",
            "mve_selected_legs": [
                {"event_ticker":"EVT-1","market_ticker":"MKT-1","side":"yes"}
            ]
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::Communications { msg, .. }) => match msg {
            WsCommunications::RfqCreated(rfq) => {
                assert_eq!(rfq.id, "rfq_123");
                assert_eq!(rfq.market_ticker, "FED-23DEC-T3.00");
                assert!(matches!(
                    rfq.mve_selected_legs.as_ref().unwrap()[0].side,
                    Some(YesNo::Yes)
                ));
            }
            other => panic!("unexpected communications payload: {:?}", other),
        },
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_rfq_deleted_message_parses() {
    let json = r#"{
        "type": "rfq_deleted",
        "msg": {
            "id": "rfq_124",
            "creator_id": "creator",
            "market_ticker": "FED-23DEC-T3.00",
            "deleted_ts": "2024-12-01T10:05:00Z"
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::Communications { msg, .. }) => match msg {
            WsCommunications::RfqDeleted(rfq) => {
                assert_eq!(rfq.id, "rfq_124");
                assert_eq!(rfq.creator_id, "creator");
            }
            other => panic!("unexpected communications payload: {:?}", other),
        },
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_quote_created_message_parses() {
    let json = r#"{
        "type": "quote_created",
        "msg": {
            "quote_id": "q-1",
            "rfq_id": "rfq-1",
            "quote_creator_id": "creator",
            "market_ticker": "FED-23DEC-T3.00",
            "yes_bid_dollars": "0.50",
            "no_bid_dollars": "0.50",
            "yes_contracts_offered_fp": "100.00",
            "created_ts": "2024-12-01T10:06:00Z"
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::Communications { msg, .. }) => match msg {
            WsCommunications::QuoteCreated(quote) => {
                assert_eq!(quote.quote_id, "q-1");
                assert_eq!(quote.yes_bid_dollars, "0.50");
                assert_eq!(quote.yes_contracts_offered_fp.as_deref(), Some("100.00"));
            }
            other => panic!("unexpected communications payload: {:?}", other),
        },
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_quote_accepted_message_parses() {
    let json = r#"{
        "type": "quote_accepted",
        "msg": {
            "quote_id": "q-2",
            "rfq_id": "rfq-2",
            "quote_creator_id": "creator",
            "market_ticker": "FED-23DEC-T3.00",
            "yes_bid_dollars": "0.51",
            "no_bid_dollars": "0.49",
            "accepted_side": "yes",
            "contracts_accepted_fp": "10.00"
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::Communications { msg, .. }) => match msg {
            WsCommunications::QuoteAccepted(quote) => {
                assert_eq!(quote.quote_id, "q-2");
                assert!(matches!(quote.accepted_side, Some(YesNo::Yes)));
                assert_eq!(quote.contracts_accepted_fp.as_deref(), Some("10.00"));
            }
            other => panic!("unexpected communications payload: {:?}", other),
        },
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_quote_executed_message_parses() {
    let json = r#"{
        "type": "quote_executed",
        "msg": {
            "quote_id": "q-3",
            "rfq_id": "rfq-3",
            "quote_creator_id": "creator",
            "rfq_creator_id": "rfq_creator",
            "order_id": "order-1",
            "client_order_id": "client-1",
            "market_ticker": "FED-23DEC-T3.00",
            "executed_ts": "2024-12-01T10:07:00Z"
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::Communications { msg, .. }) => match msg {
            WsCommunications::QuoteExecuted(quote) => {
                assert_eq!(quote.quote_id, "q-3");
                assert_eq!(quote.order_id, "order-1");
            }
            other => panic!("unexpected communications payload: {:?}", other),
        },
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_multivariate_message_parses() {
    let json = r#"{
        "type": "multivariate_lookup",
        "msg": {
            "collection_ticker": "COLL-1",
            "event_ticker": "EVT-1",
            "market_ticker": "MKT-1",
            "selected_markets": [
                {"event_ticker":"EVT-1","market_ticker":"MKT-1","side":"yes"}
            ]
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::Multivariate { msg, .. }) => {
            assert_eq!(msg.collection_ticker, "COLL-1");
            assert!(matches!(msg.selected_markets[0].side, YesNo::Yes));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_order_group_updates_message_parses() {
    let json = r#"{
        "type": "order_group_updates",
        "msg": {"event_type":"limit_updated","order_group_id":"og-1","contracts_limit_fp":"150.00"}
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::OrderGroupUpdates { msg, .. }) => {
            assert_eq!(msg.order_group_id, "og-1");
            assert!(matches!(
                msg.event_type,
                WsOrderGroupEventType::LimitUpdated
            ));
            assert_eq!(msg.contracts_limit_fp.as_deref(), Some("150.00"));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_user_order_message_parses() {
    let json = r#"{
        "type": "user_order",
        "sid": 2,
        "msg": {
            "order_id": "12e9c0eb-ff95-49d8-9d95-304f331093d2",
            "user_id": "f57fcae6-fb98-4c89-bad2-55c85e80f89a",
            "ticker": "INXD-26FEB14-T19000",
            "status": "resting",
            "side": "yes",
            "is_yes": true,
            "yes_price_dollars": "0.5500",
            "fill_count_fp": "0.00",
            "remaining_count_fp": "10.00",
            "initial_count_fp": "10.00",
            "taker_fill_cost_dollars": "0.0000",
            "maker_fill_cost_dollars": "0.0000",
            "taker_fees_dollars": "0.0000",
            "maker_fees_dollars": "0.0000",
            "client_order_id": "abc-123",
            "created_time": "2026-02-14T12:00:00Z",
            "created_ts_ms": 1771070400000
        }
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::Data(WsDataMessageV2::UserOrder { sid, msg, .. }) => {
            assert_eq!(sid, Some(2));
            assert_eq!(msg.ticker, "INXD-26FEB14-T19000");
            assert_eq!(msg.is_yes, Some(true));
            assert_eq!(msg.yes_price_dollars.as_deref(), Some("0.5500"));
            assert_eq!(msg.remaining_count_fp.as_deref(), Some("10.00"));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn ws_list_subscriptions_parses_from_subscriptions_field() {
    let json = r#"{
        "id": 2,
        "type": "list_subscriptions",
        "subscriptions": [
            {"sid": 10, "channels": ["ticker"], "market_tickers": ["TEST"]}
        ]
    }"#;

    let env: WsEnvelope = serde_json::from_str(json).unwrap();
    let msg = env.into_message().unwrap();
    match msg {
        WsMessageV2::ListSubscriptions { id, subscriptions } => {
            assert_eq!(id, Some(2));
            assert_eq!(subscriptions.len(), 1);
            assert_eq!(subscriptions[0].sid, 10);
        }
        other => panic!("unexpected: {:?}", other),
    }
}
