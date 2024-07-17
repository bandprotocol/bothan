use std::time::Duration;

use tracing_subscriber::fmt::init;

use bothan_coingecko::CoinGeckoWorkerBuilder;
use bothan_core::worker::AssetWorker;

#[tokio::main]
async fn main() {
    init();
    let worker_result = CoinGeckoWorkerBuilder::default().build().await;

    if let Ok(worker) = worker_result {
        worker
            .add_query_ids(vec!["bitcoin".to_string(), "ethereum".to_string()])
            .await
            .unwrap();
        loop {
            let data = worker.get_assets(&["bitcoin", "ethereum"]).await;
            println!("{:?}", data);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
