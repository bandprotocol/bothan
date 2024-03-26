use std::time::Duration;

use tracing_subscriber::fmt::init;

use bothan_core::service::Service;
use bothan_htx::HtxServiceBuilder;

#[tokio::main]
async fn main() {
    init();
    let service_result = HtxServiceBuilder::default().build().await;

    if let Ok(mut service) = service_result {
        loop {
            let data = service
                .get_price_data(&["btcusdt", "ethusdt", "bandusdt"])
                .await;
            println!("{:?}", data);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
