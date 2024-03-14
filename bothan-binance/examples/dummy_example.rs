use tracing_subscriber::fmt::init;

use bothan_binance::BinanceServiceBuilder;
use bothan_core::service::Service;

#[tokio::main]
async fn main() {
    init();

    let service = BinanceServiceBuilder::default().build().await;
    if let Ok(mut service) = service {
        loop {
            let data = service.get_price_data(&["btcusdt", "ethusdt"]).await;
            println!("{:?}", data);
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}
