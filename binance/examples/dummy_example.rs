use std::process::exit;

use tracing_subscriber::fmt::init;

use poc_binance::{BinanceService, BinanceWebsocket};

#[tokio::main]
async fn main() {
    init();

    let mut ws = BinanceWebsocket::default();
    let _ = ws.connect().await;
    if let Ok(mut service) = BinanceService::new(ws).await {
        for _ in 0..=20 {
            service.get_price_data(&["asdfasdcd"]).await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let data = service.get_price_data(&["asdfasdcd"]).await;
            println!("price: {:?}", data);
        }
    }

    exit(0);
}
