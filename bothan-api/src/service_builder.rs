use bothan_binance::BinanceServiceBuilder;
use bothan_coingecko::CoinGeckoServiceBuilder;
use bothan_coinmarketcap::CoinMarketCapServiceBuilder;
use bothan_cryptocompare::CryptoCompareServiceBuilder;
use bothan_htx::HtxServiceBuilder;

use crate::config::AppConfig;
use crate::manager::price_service::manager::PriceServiceManager;

pub async fn build_services(config: AppConfig, manager: &mut PriceServiceManager) {
    let binance = BinanceServiceBuilder::new(config.source.binance)
        .build()
        .await
        .unwrap();
    manager
        .add_service("binance".to_string(), Box::new(binance))
        .await;

    let coingecko = CoinGeckoServiceBuilder::new(config.source.coingecko)
        .build()
        .await
        .unwrap();
    manager
        .add_service("coingecko".to_string(), Box::new(coingecko))
        .await;

    let coinmarketcap = CoinMarketCapServiceBuilder::new(config.source.coinmarketcap)
        .build()
        .await
        .unwrap();
    manager
        .add_service("coinmarketcap".to_string(), Box::new(coinmarketcap))
        .await;

    let cryptocompare = CryptoCompareServiceBuilder::new(config.source.cryptocompare)
        .build()
        .await
        .unwrap();
    manager
        .add_service("cryptocompare".to_string(), Box::new(cryptocompare))
        .await;

    let htx = HtxServiceBuilder::new(config.source.htx)
        .build()
        .await
        .unwrap();
    manager.add_service("htx".to_string(), Box::new(htx)).await;
}
