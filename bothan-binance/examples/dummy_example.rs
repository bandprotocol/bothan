use bothan_binance::BinanceService;
use bothan_core::service::Service;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();

    if let Ok(mut service) = BinanceService::default().await {
        loop {
            let data = service.get_price_data(&["btcusdt", "ethusdt"]).await;
            println!("{:?}", data);
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}
