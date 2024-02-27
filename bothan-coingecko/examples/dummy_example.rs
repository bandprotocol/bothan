use std::time::Duration;

use tracing_subscriber::fmt::init;

use bothan_coingecko::CoinGeckoServiceBuilder;
use bothan_core::service::Service;

#[tokio::main]
async fn main() {
    init();
    let service_result = CoinGeckoServiceBuilder::default()
        .set_update_supported_assets_interval(Duration::from_secs(600))
        .build()
        .await;

    if let Ok(mut service) = service_result {
        loop {
            let data = service.get_price_data(&["bitcoin", "ethereum"]).await;
            println!("{:?}", data);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
