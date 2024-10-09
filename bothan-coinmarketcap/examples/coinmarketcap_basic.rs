use std::time::Duration;

use tokio::time::sleep;
use tracing_subscriber::fmt::init;

use bothan_coinmarketcap::{CoinMarketCapWorkerBuilder, CoinMarketCapWorkerBuilderOpts};
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
    let worker_store = store.create_worker_store(CoinMarketCapWorkerBuilder::worker_name());
    let opts = CoinMarketCapWorkerBuilderOpts::default();

    let worker = CoinMarketCapWorkerBuilder::new(worker_store, opts)
        .with_api_key("API_KEY_HERE")
        .build()
        .await
        .unwrap();

    worker
        .set_query_ids(vec!["1".to_string(), "1027".to_string()])
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    loop {
        let btc_data = worker.get_asset("1").await;
        let eth_data = worker.get_asset("1027").await;
        println!("{:?} {:?}", btc_data, eth_data);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
