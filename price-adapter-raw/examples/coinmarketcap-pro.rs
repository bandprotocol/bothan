use price_adapter_raw::CoinMarketCap;

#[tokio::main]
async fn main() {
    let coingecko = CoinMarketCap::new_with_api_key("$API_KEY".into());
    let queries = vec!["ethereum"];
    let prices = coingecko.get_prices(&queries).await;
    println!("prices: {:?}", prices);
}
