use tracing_subscriber::fmt::init;

use bothan_binance::{BinanceService, BinanceWebsocket};

#[tokio::main]
async fn main() {
    init();

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    if let Ok(mut service) = BinanceService::new(None).await {
        loop {
            let data = service.get_price_data(&["btcusdt", "ethusdt"]).await;
            println!("{:?}", data);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
