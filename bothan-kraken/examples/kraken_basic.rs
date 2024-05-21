use tracing_subscriber::fmt::init;

use bothan_core::service::Service;
use bothan_kraken::service::builder::KrakenServiceBuilder;

#[tokio::main]
async fn main() {
    init();

    let service = KrakenServiceBuilder::default().build().await;

    if let Ok(mut service) = service {
        loop {
            let data = service.get_price_data(&["BTC/USD", "ETH/USD"]).await;
            println!("{:?}", data);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
