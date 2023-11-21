use price_adapter::types::PriceAdapter;
use price_adapter::CoinGecko;

#[tokio::main]
async fn main() {
    let coingecko = CoinGecko::default(None).unwrap();
    let queries = vec!["ETH", "BAND"];
    let prices = coingecko.get_prices(&queries).await;
    println!("prices: {:?}", prices);
}
