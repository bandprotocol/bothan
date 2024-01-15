use price_adapter_raw::{BinanceWebsocket, BinanceWebsocketService};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let binance_ws = BinanceWebsocket::new("wss://stream.binance.com:9443");
    let service = BinanceWebsocketService::new(binance_ws);
    service.start(&["ethbtc", "btcusdt"]).await.unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;
    let price = service.get_prices(&["btcusdt"]).await;
    println!("price: {:?}", price);
}
