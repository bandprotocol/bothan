use bothan_binance::BinanceServiceBuilder;
use bothan_coingecko::CoinGeckoServiceBuilder;
use bothan_coinmarketcap::CoinMarketCapServiceBuilder;
use bothan_cryptocompare::CryptoCompareServiceBuilder;
use bothan_htx::HtxServiceBuilder;

use crate::config::AppConfig;
use crate::manager::price_service::manager::PriceServiceManager;
use crate::utils::add_service;

pub async fn initialize_services(config: AppConfig, manager: &mut PriceServiceManager) {
    add_service!(
        manager,
        BinanceServiceBuilder,
        "binance",
        config.source.binance
    );

    add_service!(
        manager,
        CoinGeckoServiceBuilder,
        "coingecko",
        config.source.coingecko
    );

    add_service!(
        manager,
        CoinMarketCapServiceBuilder,
        "coinmarketcap",
        config.source.coinmarketcap
    );

    add_service!(
        manager,
        CryptoCompareServiceBuilder,
        "cryptocompare",
        config.source.cryptocompare
    );

    add_service!(manager, HtxServiceBuilder, "htx", config.source.htx);
}
