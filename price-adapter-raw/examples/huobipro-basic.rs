use price_adapter_raw::HuobiPro;

#[tokio::main]
async fn main() {
    let huobipro = HuobiPro::new();
    let queries = vec!["btcusdt"];
    let prices = huobipro.get_prices(&queries).await;
    println!("prices: {:?}", prices);
}
