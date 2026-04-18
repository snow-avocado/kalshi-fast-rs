/// Example: consume raw WS events and parse a borrowed view
use kalshi_fast::{
    KalshiAuth, KalshiEnvironment, KalshiWsClient, WsChannelV2, WsDataMessageRef, WsEvent,
    WsMessageRef, WsReaderConfig, WsReaderMode, WsReconnectConfig, WsSubscriptionParamsV2,
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
        channels: vec![WsChannelV2::Ticker],
        ..Default::default()
    })
    .await?;

    let events = ws
        .start_reader_v2(WsReaderConfig {
            buffer_size: 1024,
            mode: WsReaderMode::Raw,
        })
        .await?;

    while let Some(event) = events.next().await {
        match event {
            WsEvent::Raw(raw) => {
                let msg = raw.parse_borrowed()?;
                if let WsMessageRef::Data(WsDataMessageRef::Ticker { msg, .. }) = msg {
                    println!(
                        "type=ticker market={} price={}",
                        msg.market_ticker, msg.price_dollars
                    );
                }
            }
            WsEvent::Reconnected { attempt } => println!("Reconnected (attempt {})", attempt),
            WsEvent::Disconnected { error } => {
                println!("Disconnected: {:?}", error);
                break;
            }
            WsEvent::Message(_) => {}
        }
    }

    Ok(())
}
