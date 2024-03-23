use std::time::Duration;

use tracing_subscriber::fmt::init;

use bothan_coinmarketcap::CoinMarketCapServiceBuilder;
use bothan_core::service::Service;

#[tokio::main]
async fn main() {
    init();
    let service_result = CoinMarketCapServiceBuilder::default()
        .set_api_key("API_KEY_HERE")
        .build()
        .await;

    if let Ok(mut service) = service_result {
        loop {
            let data = service.get_price_data(&["1", "2", "3"]).await;
            println!("{:?}", data);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
