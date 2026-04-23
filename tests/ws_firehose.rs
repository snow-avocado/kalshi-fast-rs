#![cfg(feature = "live-prod-tests")]

mod common;

use kalshi_fast::{
    KalshiEnvironment, KalshiWsLowLevelClient, WsChannelV2, WsDataMessageV2, WsMessageV2,
    WsSubscriptionParamsV2,
};

async fn subscribe_and_expect_data<F>(
    ws: &mut KalshiWsLowLevelClient,
    params: WsSubscriptionParamsV2,
    match_data: F,
) where
    F: Fn(&WsMessageV2) -> bool,
{
    let subscribe_id = ws.subscribe_v2(params).await.expect("subscribe_v2 failed");

    let _sid = common::wait_for_subscribed(ws, subscribe_id).await;

    common::wait_for_message(ws, common::CHANNEL_TIMEOUT, match_data).await;
}

// ---- OrderbookDelta: firehose (no filters) must be rejected ----

#[tokio::test]
async fn orderbook_firehose_rejected_by_client_validation() {
    // Client-side validation blocks OrderbookDelta without market_ticker(s);
    // this encodes the "orderbook_delta requires filter" assumption without
    // needing a live connection. See src/ws/types/subscription.rs:237.
    let mut ws = common::connect_demo_ws().await;

    let result = ws
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            ..Default::default()
        })
        .await;

    assert!(
        result.is_err(),
        "orderbook_delta firehose must be rejected, got Ok"
    );
}

// ---- OrderbookDelta: with market_tickers, accepted on both envs ----

async fn assert_orderbook_with_filter_accepted(env: KalshiEnvironment) {
    let client = common::rest_client(env.clone());
    let ticker = common::first_open_market_ticker(&client).await;

    let mut ws = common::connect_ws(env).await;

    subscribe_and_expect_data(
        &mut ws,
        WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec![ticker]),
            send_initial_snapshot: Some(true),
            ..Default::default()
        },
        |msg| {
            matches!(
                msg,
                WsMessageV2::Data(
                    WsDataMessageV2::OrderbookSnapshot { .. }
                        | WsDataMessageV2::OrderbookDelta { .. }
                )
            )
        },
    )
    .await;
}

#[tokio::test]
async fn orderbook_filtered_accepted_demo() {
    assert_orderbook_with_filter_accepted(common::demo_env()).await;
}

#[tokio::test]
async fn orderbook_filtered_accepted_prod() {
    assert_orderbook_with_filter_accepted(common::prod_env()).await;
}

// ---- Trade: firehose (no filters) accepted on both envs ----

async fn assert_trade_firehose_accepted(env: KalshiEnvironment) {
    let mut ws = common::connect_ws(env).await;
    subscribe_and_expect_data(
        &mut ws,
        WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Trade],
            ..Default::default()
        },
        |msg| matches!(msg, WsMessageV2::Data(WsDataMessageV2::Trade { .. })),
    )
    .await;
}

#[tokio::test]
async fn trade_firehose_demo() {
    assert_trade_firehose_accepted(common::demo_env()).await;
}

#[tokio::test]
async fn trade_firehose_prod() {
    assert_trade_firehose_accepted(common::prod_env()).await;
}

// ---- Trade: with market_tickers filter, accepted on both envs ----

async fn assert_trade_with_filter_accepted(env: KalshiEnvironment) {
    let client = common::rest_client(env.clone());
    let ticker = common::first_open_market_ticker(&client).await;

    let mut ws = common::connect_ws(env).await;
    subscribe_and_expect_data(
        &mut ws,
        WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Trade],
            market_tickers: Some(vec![ticker]),
            ..Default::default()
        },
        |msg| matches!(msg, WsMessageV2::Data(WsDataMessageV2::Trade { .. })),
    )
    .await;
}

#[tokio::test]
async fn trade_with_filter_demo() {
    assert_trade_with_filter_accepted(common::demo_env()).await;
}

#[tokio::test]
async fn trade_with_filter_prod() {
    assert_trade_with_filter_accepted(common::prod_env()).await;
}

// ---- Ticker: firehose (no filters) accepted on both envs ----

async fn assert_ticker_firehose_accepted(env: KalshiEnvironment) {
    let mut ws = common::connect_ws(env).await;
    subscribe_and_expect_data(
        &mut ws,
        WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            ..Default::default()
        },
        |msg| matches!(msg, WsMessageV2::Data(WsDataMessageV2::Ticker { .. })),
    )
    .await;
}

#[tokio::test]
async fn ticker_firehose_demo() {
    assert_ticker_firehose_accepted(common::demo_env()).await;
}

#[tokio::test]
async fn ticker_firehose_prod() {
    assert_ticker_firehose_accepted(common::prod_env()).await;
}

// ---- Ticker: with market_tickers filter, accepted on both envs ----

async fn assert_ticker_with_filter_accepted(env: KalshiEnvironment) {
    let client = common::rest_client(env.clone());
    let ticker = common::first_open_market_ticker(&client).await;

    let mut ws = common::connect_ws(env).await;
    subscribe_and_expect_data(
        &mut ws,
        WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            market_tickers: Some(vec![ticker]),
            ..Default::default()
        },
        |msg| matches!(msg, WsMessageV2::Data(WsDataMessageV2::Ticker { .. })),
    )
    .await;
}

#[tokio::test]
async fn ticker_with_filter_demo() {
    assert_ticker_with_filter_accepted(common::demo_env()).await;
}

#[tokio::test]
async fn ticker_with_filter_prod() {
    assert_ticker_with_filter_accepted(common::prod_env()).await;
}
