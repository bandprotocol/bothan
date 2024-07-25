use tracing_subscriber::fmt::init;

use bothan_coinbase::CoinbaseWorkerBuilder;
use bothan_core::worker::AssetWorker;

#[tokio::main]
async fn main() {
    init();
    let worker = CoinbaseWorkerBuilder::default().build().await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    worker
        .add_query_ids(vec![
            "BTC-USD".to_string(),
            "ETH-USD".to_string(),
            "USDT-USD".to_string(),
        ])
        .await
        .unwrap();
    loop {
        let data = worker.get_assets(&["BTC-USD", "ETH-USD", "USDT-USD"]).await;
        println!("{:?}", data);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
