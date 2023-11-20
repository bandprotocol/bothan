use price_adapter::mapper::BandStaticMapper;
use price_adapter::CoinGecko;

#[tokio::main]
async fn main() {
    let band_static_mapper = BandStaticMapper::from_source("coingecko").unwrap();
    let coingecko = CoinGecko::new(band_static_mapper, None);
    let queries = vec!["ETH", "BAND"];
    let prices = coingecko.get_prices(&queries).await;
    println!("prices: {:?}", prices);
}
