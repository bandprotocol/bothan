use price_adapter::services::IntervalService;
use price_adapter::sources::CoinGecko;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let coingecko = CoinGecko::default(None).unwrap();
    let mut service = IntervalService::new(coingecko);
    service.start(vec!["BTC"].as_slice(), 20).await.unwrap();

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("{:?}", service.get_prices(&["BTC"]).await);
    }
}
