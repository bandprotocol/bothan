use std::sync::Arc;
use std::time::Duration;

use tokio::time::sleep;
use tracing_subscriber::fmt::init;

use bothan_coingecko::{CoinGeckoWorkerBuilder, CoinGeckoWorkerBuilderOpts};
use bothan_core::store::{Store, WorkerStore};
use bothan_core::worker::AssetWorker;

#[tokio::main]
async fn main() {
    init();
    let path = std::env::current_dir().unwrap();
    let store = Arc::new(
        Store::new(Default::default(), path.as_path())
            .await
            .unwrap(),
    );
    let worker_store = Arc::new(WorkerStore::from_store(store, "binance"));
    let opts = CoinGeckoWorkerBuilderOpts::default();

    let worker = CoinGeckoWorkerBuilder::new(worker_store, opts)
        .build()
        .await
        .unwrap();

    worker
        .add_query_ids(vec!["bitcoin".to_string(), "ethereum".to_string()])
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    loop {
        let data = worker.get_assets(&["bitcoin", "ethereum"]).await;
        println!("{:?}", data);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
