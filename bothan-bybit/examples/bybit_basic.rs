use std::time::Duration;

use tracing_subscriber::fmt::init;

use bothan_bybit::BybitServiceBuilder;
use bothan_core::service::Service;

#[tokio::main]
async fn main() {
    init();
    let service_result = BybitServiceBuilder::default().build().await;

    if let Ok(mut service) = service_result {
        loop {
            let data = service.get_price_data(&["BTCUSDT", "ETHUSDT"]).await;
            println!("{:?}", data);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
