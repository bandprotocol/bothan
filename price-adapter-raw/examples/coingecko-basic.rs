use price_adapter_raw::CoinGecko;

#[tokio::main]
async fn main() {
    let coingecko = CoinGecko::new();
    let queries = vec!["ethereum"];
    let prices = coingecko.get_prices(&queries).await;
    println!("prices: {:?}", prices);
}
