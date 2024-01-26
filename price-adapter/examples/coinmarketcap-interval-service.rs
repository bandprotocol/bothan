use price_adapter::services::IntervalService;
use price_adapter::sources::CoinMarketCap;
use price_adapter::types::{Service, Source};
use std::time::Duration;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let coinmarketcap: CoinMarketCap<price_adapter::mappers::BandStaticMapper> =
        CoinMarketCap::new_with_default("$API_KEY".into()).unwrap();
    let mut service = IntervalService::new(coinmarketcap, Duration::from_secs(20));
    service
        .start(vec!["BAND", "AVAX"].as_slice())
        .await
        .unwrap();

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("{:?}", service.get_prices(&["BAND", "AVAX"]).await);
    }
}
