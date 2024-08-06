use std::sync::Arc;
use std::time::Duration;

use tokio::time::sleep;
use tracing_subscriber::fmt::init;

use bothan_binance::{BinanceWorkerBuilder, BinanceWorkerBuilderOpts};
use bothan_core::store::{SharedStore, WorkerStore};
use bothan_core::worker::AssetWorker;

#[tokio::main]
async fn main() {
    init();
    let path = std::env::current_dir().unwrap();
    let store = Arc::new(
        SharedStore::new(Default::default(), path.as_path())
            .await
            .unwrap(),
    );
    let worker_store = Arc::new(WorkerStore::from_store(store, "binance"));
    let opts = BinanceWorkerBuilderOpts::default();

    let worker = BinanceWorkerBuilder::new(worker_store, opts)
        .build()
        .await
        .unwrap();

    worker
        .add_query_ids(vec!["btcusdt".to_string(), "ethusdt".to_string()])
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    loop {
        let data = worker.get_assets(&["btcusdt", "ethusdt"]).await;
        println!("{:?}", data);
        sleep(Duration::from_secs(5)).await;
    }
}
