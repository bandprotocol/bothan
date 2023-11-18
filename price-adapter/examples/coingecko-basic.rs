use price_adapter::coingecko::CoinGecko;
use price_adapter::mapper::BandStaticMapper;

#[tokio::main]
async fn main() {
    let band_static_mapper = Box::new(BandStaticMapper::new("coingecko".to_string()));
    let coingecko = CoinGecko::new(band_static_mapper, None);
    let queries = vec!["ETH"];
    let prices = coingecko.get_prices(&queries).await;
    println!("prices: {:?}", prices);
}
