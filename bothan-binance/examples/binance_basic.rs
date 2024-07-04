use tracing_subscriber::fmt::init;

use bothan_binance::BinanceStoreBuilder;
use bothan_core::worker::AssetWorker;

#[tokio::main]
async fn main() {
    init();
    let mut store = BinanceStoreBuilder::default().build().await.unwrap();

    store.start().await;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    store.add_query_ids(&["btcusdt", "ethusdt"]).await.unwrap();
    loop {
        let data = store.get_assets(&["btcusdt", "ethusdt"]).await;
        println!("{:?}", data);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
