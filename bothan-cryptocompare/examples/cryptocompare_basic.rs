use std::time::Duration;

use tokio::time::sleep;
use tracing_subscriber::fmt::init;

use bothan_core::registry::Registry;
use bothan_core::store::SharedStore;
use bothan_core::worker::{AssetWorker, AssetWorkerBuilder};
use bothan_cryptocompare::{CryptoCompareWorkerBuilder, CryptoCompareWorkerBuilderOpts};

#[tokio::main]
async fn main() {
    init();
    let path = std::env::current_dir().unwrap();
    let store = SharedStore::new(Registry::default().validate().unwrap(), path.as_path())
        .await
        .unwrap();
    let worker_store = store.create_worker_store(CryptoCompareWorkerBuilder::worker_name());
    let opts = CryptoCompareWorkerBuilderOpts::default();

    let worker = CryptoCompareWorkerBuilder::new(worker_store, opts)
        .build()
        .await
        .unwrap();

    worker.set_query_ids(vec!["BTC".to_string()]).await.unwrap();

    sleep(Duration::from_secs(2)).await;

    loop {
        let data = worker.get_asset("BTC").await;
        println!("{:?}", data);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
