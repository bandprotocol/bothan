use price_adapter::services::WebsocketService;
use price_adapter::sources::BinanceWebsocket;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let binance_websocket = BinanceWebsocket::new_with_default().unwrap();
    let mut service = WebsocketService::new(binance_websocket);
    service.start(vec!["BTC"].as_slice()).await.unwrap();

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("{:?}", service.get_prices(&["BTC"]).await);
    }
}
