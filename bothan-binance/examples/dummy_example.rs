use tracing_subscriber::fmt::init;

use bothan_binance::{BinanceService, BinanceWebsocket};

#[tokio::main]
async fn main() {
    init();

    let mut ws = BinanceWebsocket::default();
    let _ = ws.connect().await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let duration = tokio::time::Duration::from_secs(100);
    if let Ok(mut service) = BinanceService::new(ws).await {
        loop {
            let data = service.get_price_data(&["btcusdt", "ethusdt"]).await;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
