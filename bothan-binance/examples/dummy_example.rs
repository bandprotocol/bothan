use bothan_binance::Binance;
use bothan_core::service::Service;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();

    if let Ok(mut service) = Binance::default().await {
        loop {
            let data = service.get_price_data(&["btcusdt", "ethusdt"]).await;
            println!("{:?}", data);
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }
}
