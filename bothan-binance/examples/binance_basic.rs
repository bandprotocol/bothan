use tracing_subscriber::fmt::init;

use bothan_binance::BinanceWorkerBuilder;
use bothan_core::worker::AssetWorker;

#[tokio::main]
async fn main() {
    init();
    let worker = BinanceWorkerBuilder::default().build().await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    worker.add_query_ids(&["btcusdt", "ethusdt"]).await.unwrap();
    loop {
        let data = worker.get_assets(&["btcusdt", "ethusdt"]).await;
        println!("{:?}", data);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
