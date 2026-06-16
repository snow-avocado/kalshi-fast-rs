/// Example of margin/perpetuals REST API usage for KXBTCPERP1:
///   1. Check if margin trading is enabled
///   2. List active margin markets
///   3. Fetch balance and positions
use kalshi_fast::{
    GetMarginPositionsParams, KalshiAuth, KalshiEnvironment, KalshiRestClient, MarginMarketStatus,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let env = KalshiEnvironment::demo();
    let auth = KalshiAuth::from_pem_file(
        std::env::var("KALSHI_KEY_ID")?,
        std::env::var("KALSHI_PRIVATE_KEY_PATH")?,
    )?;
    let client = KalshiRestClient::new(env).with_auth(auth);

    let enabled = client.get_margin_enabled().await?;
    println!("margin trading enabled: {}", enabled.enabled);

    let markets = client
        .get_margin_markets(Some(MarginMarketStatus::Active))
        .await?;
    println!("active margin markets: {}", markets.markets.len());

    if let Some(market) = markets.markets.first() {
        println!(
            "  → {}: price={}, volume={}",
            market.ticker,
            market.price.as_deref().unwrap_or("-"),
            market.volume.as_deref().unwrap_or("-"),
        );
    }

    let balance = client.get_margin_balance(Some(true)).await?;
    println!("settled funds: {}", balance.settled_funds);
    for sub in &balance.subaccount_balances {
        println!(
            "  subaccount {}: equity={}, available={}",
            sub.subaccount,
            sub.account_equity,
            sub.available_balance.as_deref().unwrap_or("-"),
        );
    }

    let positions = client
        .get_margin_positions(GetMarginPositionsParams::default())
        .await?;
    println!("open positions: {}", positions.positions.len());

    Ok(())
}
