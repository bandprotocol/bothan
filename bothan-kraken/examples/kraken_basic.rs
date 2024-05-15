use bothan_core::service::Service;
use bothan_kraken::service::builder::KrakenServiceBuilder;

#[tokio::main]
async fn main() {
    // init();
    tracing_subscriber::fmt().with_env_filter("debug").init();

    let service = KrakenServiceBuilder::default().build().await;

    if let Ok(mut service) = service {
        let data = service.get_price_data(&["BTC/USD", "ETH/USD"]).await;
        println!("{:?}", data);
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
