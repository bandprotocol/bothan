use price_adapter::sources::CoinMarketCap;
use price_adapter::types::Source;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let coingecko = CoinMarketCap::new_with_default("$API_KEY".into()).unwrap();
    let queries = vec!["BAND", "AVAX"];
    let prices: Vec<Result<price_adapter::types::PriceInfo, price_adapter::error::Error>> =
        coingecko.get_prices(&queries).await;
    println!("prices: {:?}", prices);
}
