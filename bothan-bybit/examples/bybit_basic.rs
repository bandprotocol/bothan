use std::time::Duration;

use tokio::time::sleep;
use tracing_subscriber::fmt::init;

use bothan_bybit::{BybitWorkerBuilder, BybitWorkerBuilderOpts};
use bothan_core::registry::Registry;
use bothan_core::store::SharedStore;
use bothan_core::worker::{AssetWorker, AssetWorkerBuilder};

#[tokio::main]
async fn main() {
    init();
    let path = std::env::current_dir().unwrap();
    let registry = Registry::default().validate().unwrap();
    let store = SharedStore::new(registry, path.as_path()).await.unwrap();

    let worker_store = store.create_worker_store(BybitWorkerBuilder::worker_name());
    let opts = BybitWorkerBuilderOpts::default();

    let worker = BybitWorkerBuilder::new(worker_store, opts)
        .build()
        .await
        .unwrap();

    worker
        .set_query_ids(vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()])
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    loop {
        let btc_data = worker.get_asset("BTCUSDT").await;
        let eth_data = worker.get_asset("ETHUSDT").await;
        println!("{:?} {:?}", btc_data, eth_data);
        sleep(Duration::from_secs(5)).await;
    }
}
