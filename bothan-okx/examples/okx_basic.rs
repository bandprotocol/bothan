use tracing_subscriber::fmt::init;

use bothan_core::worker::AssetWorker;
use bothan_okx::OkxWorkerBuilder;

#[tokio::main]
async fn main() {
    init();
    let worker = OkxWorkerBuilder::default().build().await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    worker
        .add_query_ids(vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()])
        .await
        .unwrap();
    loop {
        let data = worker.get_assets(&["BTC-USDT", "ETH-USDT"]).await;
        println!("{:?}", data);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
