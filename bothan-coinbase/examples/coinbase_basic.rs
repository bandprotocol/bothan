use tracing_subscriber::fmt::init;

use bothan_coinbase::service::builder::CoinbaseServiceBuilder;
use bothan_core::service::Service;

#[tokio::main]
async fn main() {
    init();

    let service = CoinbaseServiceBuilder::default().build().await;
    if let Ok(mut service) = service {
        loop {
            let data = service
                .get_price_data(&["BTC-USD", "ETH-USD", "USDT-USD"])
                .await;
            println!("{:?}", data);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
