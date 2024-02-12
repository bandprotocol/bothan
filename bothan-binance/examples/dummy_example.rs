use tracing_subscriber::fmt::init;

use bothan_binance::BinanceService;

#[tokio::main]
async fn main() {
    init();

    if let Ok(mut service) = BinanceService::default().await {
        loop {
            let data = service.get_price_data(&["btcusdt", "ethusdt"]).await;
            println!("{:?}", data);
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }
}
