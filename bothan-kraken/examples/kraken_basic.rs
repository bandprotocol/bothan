use tracing_subscriber::fmt::init;

use bothan_core::worker::AssetWorker;
use bothan_kraken::KrakenWorkerBuilder;

#[tokio::main]
async fn main() {
    init();
    let worker = KrakenWorkerBuilder::default().build().await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    worker
        .add_query_ids(vec!["BTC/USD".to_string(), "ETH/USD".to_string()])
        .await
        .unwrap();
    loop {
        let data = worker.get_assets(&["BTC/USD", "ETH/USD"]).await;
        println!("{:?}", data);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
