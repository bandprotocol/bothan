use futures_util::StreamExt;
use price_adapter::sources::BinanceWebsocket;
use price_adapter::types::WebSocketSource;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut binance_websocket = BinanceWebsocket::new_with_default().unwrap();
    let symbols = vec!["ETH", "BTC"];

    binance_websocket.connect().await.unwrap();
    binance_websocket
        .subscribe(symbols.as_slice())
        .await
        .unwrap();

    while let Some(data) = binance_websocket.next().await {
        println!("{:?}", data);
    }
}
