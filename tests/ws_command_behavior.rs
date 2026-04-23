use std::net::SocketAddr;
use std::sync::OnceLock;
use std::time::Duration;

use futures::StreamExt;
use kalshi_fast::{
    KalshiAuth, KalshiEnvironment, KalshiError, KalshiWsLowLevelClient, WsChannelV2,
    WsSubscriptionParamsV2, WsUpdateAction, WsUpdateSubscriptionParamsV2,
};
use rand::rngs::OsRng;
use rsa::RsaPrivateKey;
use rsa::pkcs8::{EncodePrivateKey, LineEnding};
use serde_json::{Value, json};
use tokio::net::TcpListener;
use tokio::time::sleep;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

fn test_auth() -> KalshiAuth {
    static TEST_PEM: OnceLock<String> = OnceLock::new();

    let pem = TEST_PEM.get_or_init(|| {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("generate test private key");
        private_key
            .to_pkcs8_pem(LineEnding::LF)
            .expect("encode test private key")
            .to_string()
    });

    KalshiAuth::from_pem_str("test-key-id", pem).expect("load auth from generated PEM")
}

fn test_env(addr: SocketAddr) -> KalshiEnvironment {
    KalshiEnvironment {
        rest_origin: Url::parse("http://127.0.0.1/").expect("url"),
        ws_url: format!("ws://{addr}"),
    }
}

async fn read_single_text_payload(listener: TcpListener) -> Value {
    let (stream, _) = listener.accept().await.expect("accept tcp");
    let mut ws = accept_async(stream).await.expect("accept websocket");

    let frame = ws.next().await.expect("frame").expect("ok frame");
    let text = match frame {
        Message::Text(text) => text.to_string(),
        other => panic!("expected text frame, got {other:?}"),
    };

    serde_json::from_str(&text).expect("valid json payload")
}

#[tokio::test]
async fn low_level_subscribe_serializes_singular_market_ticker() {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("addr");

    let server = tokio::spawn(async move {
        let payload = read_single_text_payload(listener).await;
        assert_eq!(payload["cmd"], json!("subscribe"));
        assert_eq!(payload["params"]["channels"], json!(["ticker"]));
        assert_eq!(payload["params"]["market_ticker"], json!("TEST"));
        assert!(payload["params"].get("market_tickers").is_none());
    });

    let mut client = KalshiWsLowLevelClient::connect_authenticated(test_env(addr), test_auth())
        .await
        .expect("connect");
    client
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            market_ticker: Some("TEST".to_string()),
            ..Default::default()
        })
        .await
        .expect("subscribe");

    server.await.expect("server");
}

#[tokio::test]
async fn low_level_subscribe_orderbook_delta_serializes_plural_market_tickers() {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("addr");

    let server = tokio::spawn(async move {
        let payload = read_single_text_payload(listener).await;
        assert_eq!(payload["cmd"], json!("subscribe"));
        assert_eq!(payload["params"]["channels"], json!(["orderbook_delta"]));
        assert_eq!(payload["params"]["market_tickers"], json!(["TEST"]));
        assert_eq!(payload["params"]["send_initial_snapshot"], json!(true));
        assert!(payload["params"].get("market_ticker").is_none());
        assert!(payload["params"].get("market_id").is_none());
        assert!(payload["params"].get("market_ids").is_none());
    });

    let mut client = KalshiWsLowLevelClient::connect_authenticated(test_env(addr), test_auth())
        .await
        .expect("connect");
    client
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::OrderbookDelta],
            market_tickers: Some(vec!["TEST".to_string()]),
            send_initial_snapshot: Some(true),
            ..Default::default()
        })
        .await
        .expect("subscribe");

    server.await.expect("server");
}

#[tokio::test]
async fn low_level_update_subscription_serializes_skip_ticker_ack() {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("addr");

    let server = tokio::spawn(async move {
        let payload = read_single_text_payload(listener).await;
        assert_eq!(payload["cmd"], json!("update_subscription"));
        assert_eq!(payload["params"]["action"], json!("add_markets"));
        assert_eq!(payload["params"]["sid"], json!(42));
        assert_eq!(payload["params"]["market_tickers"], json!(["TEST", "ALT"]));
        assert_eq!(payload["params"]["skip_ticker_ack"], json!(true));
        assert!(payload["params"].get("sids").is_none());
    });

    let mut client = KalshiWsLowLevelClient::connect_authenticated(test_env(addr), test_auth())
        .await
        .expect("connect");
    client
        .update_subscription_v2(WsUpdateSubscriptionParamsV2 {
            action: WsUpdateAction::AddMarkets,
            sid: Some(42),
            sids: None,
            market_ticker: None,
            market_tickers: Some(vec!["TEST".to_string(), "ALT".to_string()]),
            market_id: None,
            market_ids: None,
            send_initial_snapshot: None,
            skip_ticker_ack: Some(true),
        })
        .await
        .expect("update subscription");

    server.await.expect("server");
}

#[tokio::test]
async fn low_level_subscribe_rejects_both_market_ticker_forms() {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("addr");

    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("accept tcp");
        let _ws = accept_async(stream).await.expect("accept websocket");
        sleep(Duration::from_millis(100)).await;
    });

    let mut client = KalshiWsLowLevelClient::connect_authenticated(test_env(addr), test_auth())
        .await
        .expect("connect");
    let err = client
        .subscribe_v2(WsSubscriptionParamsV2 {
            channels: vec![WsChannelV2::Ticker],
            market_ticker: Some("TEST".to_string()),
            market_tickers: Some(vec!["ALT".to_string()]),
            ..Default::default()
        })
        .await
        .expect_err("subscribe should fail");

    assert!(matches!(
        err,
        KalshiError::InvalidParams(message)
            if message.contains("market_ticker or market_tickers")
    ));

    server.await.expect("server");
}
