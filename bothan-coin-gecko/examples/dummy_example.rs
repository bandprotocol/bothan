use bothan_coin_gecko::api::CoinGeckoRest;
use bothan_coin_gecko::CoinGecko;
use bothan_core::service::Service;

use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();
    let mut cg = CoinGecko::new().await.unwrap();

    for _ in 0..100 {
        let test = cg.get_price_data(&["bitcoin"]).await;
        println!("{:?}", test);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
