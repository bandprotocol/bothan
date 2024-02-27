use bothan_binance::service::BinanceServiceBuilder;
use bothan_binance::BinanceService;
use bothan_core::service::Service;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();

    let service = BinanceServiceBuilder::new().build().await;
    if let Ok(mut service) = service {
        loop {
            let data = service.get_price_data(&["btcusdt", "ethusdt"]).await;
            println!("{:?}", data);
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}
