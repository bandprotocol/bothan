use std::time::Duration;

use tokio::time::sleep;
use tracing_subscriber::fmt::init;

use bothan_core::registry::Registry;
use bothan_core::store::SharedStore;
use bothan_core::worker::{AssetWorker, AssetWorkerBuilder};
use bothan_okx::{OkxWorkerBuilder, OkxWorkerBuilderOpts};

#[tokio::main]
async fn main() {
    init();
    let path = std::env::current_dir().unwrap();
    let registry = Registry::default().validate().unwrap();
    let store = SharedStore::new(registry, path.as_path()).await.unwrap();

    let worker_store = store.create_worker_store(OkxWorkerBuilder::worker_name());
    let opts = OkxWorkerBuilderOpts::default();

    let worker = OkxWorkerBuilder::new(worker_store, opts)
        .build()
        .await
        .unwrap();

    worker
        .set_query_ids(vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()])
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    loop {
        let btc_data = worker.get_asset("BTC-USDT").await;
        let eth_data = worker.get_asset("ETH-USDT").await;
        println!("{:?}, {:?}", btc_data, eth_data);
        sleep(Duration::from_secs(5)).await;
    }
}
