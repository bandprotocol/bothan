use tracing_subscriber::fmt::init;

use bothan_core::service::Service;
use bothan_okx::service::builder::OKXServiceBuilder;

#[tokio::main]
async fn main() {
    init();

    let service = OKXServiceBuilder::default().build().await;
    if let Ok(mut service) = service {
        loop {
            let data = service.get_price_data(&["BTC-USDT", "ETH-USDT"]).await;
            println!("{:?}", data);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
