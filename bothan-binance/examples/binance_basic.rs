use std::time::Duration;

use tokio::time::sleep;
use tracing_subscriber::fmt::init;

use bothan_binance::{BinanceWorkerBuilder, BinanceWorkerBuilderOpts};
use bothan_core::registry::Registry;
use bothan_core::store::SharedStore;
use bothan_core::worker::{AssetWorker, AssetWorkerBuilder};

#[tokio::main]
async fn main() {
    init();
    let path = std::env::current_dir().unwrap();
    let registry = Registry::default().validate().unwrap();
    let store = SharedStore::new(registry, path.as_path()).await.unwrap();

    let worker_store = store.create_worker_store("binance");
    let opts = BinanceWorkerBuilderOpts::default();

    let worker = BinanceWorkerBuilder::new(worker_store, opts)
        .build()
        .await
        .unwrap();

    worker
        .set_query_ids(vec!["btcusdt".to_string(), "ethusdt".to_string()])
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    loop {
        let data = worker.get_asset("btcusdt").await;
        println!("{:?}", data);
        sleep(Duration::from_secs(5)).await;
    }
}
