use price_adapter::services::IntervalService;
use price_adapter::sources::HuobiPro;
use price_adapter::types::{Service, Source};
use std::time::Duration;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let huobipro = HuobiPro::new_with_default().unwrap();
    let mut service = IntervalService::new(huobipro, Duration::from_secs(20));
    service.start(vec!["BTC"].as_slice()).await.unwrap();

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("{:?}", service.get_prices(&["BTC"]).await);
    }
}
