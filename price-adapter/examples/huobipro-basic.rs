use price_adapter::sources::HuobiPro;
use price_adapter::types::Source;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let huobipro = HuobiPro::new_with_default().unwrap();
    let queries = vec!["ETH", "BTC"];
    let prices = huobipro.get_prices(&queries).await;
    println!("prices: {:?}", prices);
}
