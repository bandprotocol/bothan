use std::time::Duration;

use tokio::time::sleep;
use tracing_subscriber::fmt::init;

use bothan_core::registry::Registry;
use bothan_core::store::SharedStore;
use bothan_core::worker::{AssetWorker, AssetWorkerBuilder};
use bothan_htx::{HtxWorkerBuilder, HtxWorkerBuilderOpts};

#[tokio::main]
async fn main() {
    init();
    let path = std::env::current_dir().unwrap();
    let registry = Registry::default().validate().unwrap();
    let store = SharedStore::new(registry, path.as_path()).await.unwrap();

    let worker_store = store.create_worker_store(HtxWorkerBuilder::worker_name());
    let opts = HtxWorkerBuilderOpts::default();

    let worker = HtxWorkerBuilder::new(worker_store, opts)
        .build()
        .await
        .unwrap();

    worker
        .set_query_ids(vec!["btcusdt".to_string(), "ethusdt".to_string()])
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    loop {
        let btc_data = worker.get_asset("btcusdt").await;
        let eth_data = worker.get_asset("ethusdt").await;
        println!("{:?} {:?}", btc_data, eth_data);
        sleep(Duration::from_secs(5)).await;
    }
}
