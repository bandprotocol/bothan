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
    let path = std::env::current_dir().unwrap().join("store");
    let registry = Registry::default().validate().unwrap();
    let store = SharedStore::new(registry, &path).await.unwrap();
    let worker_store = store.create_worker_store(CryptoCompareWorkerBuilder::worker_name());
    let opts = CryptoCompareWorkerBuilderOpts::default();
    let worker = CryptoCompareWorkerBuilder::new(worker_store, opts)
        .with_api_key("YOUR_API_KEY")
        .build()
        .await
        .unwrap();

    worker
        .set_query_ids(vec!["BTC-USD".to_string(), "ETH-USD".to_string()])
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    loop {
        let btc_data = worker.get_asset("BTC-USD").await;
        let eth_data = worker.get_asset("ETH-USD").await;
        println!("{:?} {:?}", btc_data, eth_data);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
