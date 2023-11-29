use price_adapter::services::IntervalService;
use price_adapter::sources::CoinGecko;
use price_adapter::types::{Service, Source};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let coingecko = CoinGecko::new_with_default(None).unwrap();
    let mut service = IntervalService::new(coingecko, Duration::from_secs(20));
    service.start(vec!["BTC"].as_slice()).await.unwrap();

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("{:?}", service.get_prices(&["BTC"]).await);
    }
}
