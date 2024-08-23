use std::time::Duration;

use tokio::time::sleep;
use tracing_subscriber::fmt::init;

use bothan_coingecko::{CoinGeckoWorkerBuilder, CoinGeckoWorkerBuilderOpts};
use bothan_core::registry::Registry;
use bothan_core::store::SharedStore;
use bothan_core::worker::{AssetWorker, AssetWorkerBuilder};

#[tokio::main]
async fn main() {
    init();
    let path = std::env::current_dir().unwrap();
    let store = SharedStore::new(Registry::default().validate().unwrap(), path.as_path())
        .await
        .unwrap();
    let worker_store = store.create_worker_store("coingecko");
    let opts = CoinGeckoWorkerBuilderOpts::default();

    let worker = CoinGeckoWorkerBuilder::new(worker_store, opts)
        .build()
        .await
        .unwrap();

    worker
        .set_query_ids(vec!["bitcoin".to_string()])
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    loop {
        let data = worker.get_asset("bitcoin").await;
        println!("{:?}", data);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
