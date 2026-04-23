/// Example of using an authenticated WS channel
///
/// This channel is explicitly called out in the docs as being authenticated
use kalshi_fast::{
    KalshiAuth, KalshiEnvironment, KalshiWsClient, WsChannelV2, WsEvent, WsReconnectConfig,
    WsSubscriptionParamsV2,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = KalshiEnvironment::demo();

    let auth = KalshiAuth::from_pem_file(
        std::env::var("KALSHI_KEY_ID")?,
        std::env::var("KALSHI_PRIVATE_KEY_PATH")?,
    )?;

    let mut ws =
        KalshiWsClient::connect_authenticated(env, auth, WsReconnectConfig::default()).await?;

    ws.subscribe_v2(WsSubscriptionParamsV2 {
        channels: vec![WsChannelV2::OrderbookDelta],
        market_tickers: Some(vec!["SOME_MARKET_TICKER".to_string()]),
        ..Default::default()
    })
    .await?;

    loop {
        match ws.next_event_v2().await? {
            WsEvent::Message(msg) => println!("{:?}", msg),
            WsEvent::Raw(_) => {}
            WsEvent::Reconnected { attempt } => println!("Reconnected (attempt {})", attempt),
            WsEvent::Disconnected { error } => {
                println!("Disconnected: {:?}", error);
                break;
            }
        }
    }

    Ok(())
}
