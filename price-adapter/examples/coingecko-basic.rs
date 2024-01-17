use price_adapter::sources::CoinGecko;
use price_adapter::types::Source;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let coingecko = CoinGecko::new_with_default(None).unwrap();
    let queries = vec!["ETH", "BAND"];
    let prices = coingecko.get_prices(&queries).await;
    println!("prices: {:?}", prices);
}
