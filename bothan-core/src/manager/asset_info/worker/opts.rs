//! Worker options for configuring asset source workers.

use {
    bothan_band, bothan_binance, bothan_bitfinex, bothan_bybit, bothan_coinbase, bothan_coingecko,
    bothan_coinmarketcap, bothan_htx, bothan_kraken, bothan_okx,
};

#[derive(Clone)]
pub enum AssetWorkerOpts {
    Binance(bothan_binance::WorkerOpts),
    Bitfinex(bothan_bitfinex::WorkerOpts),
    Bybit(bothan_bybit::WorkerOpts),
    Coinbase(bothan_coinbase::WorkerOpts),
    CoinGecko(bothan_coingecko::WorkerOpts),
    CoinMarketCap(bothan_coinmarketcap::WorkerOpts),
    Htx(bothan_htx::WorkerOpts),
    Kraken(bothan_kraken::WorkerOpts),
    Okx(bothan_okx::WorkerOpts),
    Band(bothan_band::WorkerOpts),
}

impl AssetWorkerOpts {
    pub fn name(&self) -> &str {
        match self {
            AssetWorkerOpts::Binance(_) => "binance",
            AssetWorkerOpts::Bitfinex(_) => "bitfinex",
            AssetWorkerOpts::Bybit(_) => "bybit",
            AssetWorkerOpts::Coinbase(_) => "coinbase",
            AssetWorkerOpts::CoinGecko(_) => "coingecko",
            AssetWorkerOpts::CoinMarketCap(_) => "coinmarketcap",
            AssetWorkerOpts::Htx(_) => "htx",
            AssetWorkerOpts::Kraken(_) => "kraken",
            AssetWorkerOpts::Okx(_) => "okx",
            AssetWorkerOpts::Band(opts) => opts.name(),
        }
    }
}

impl From<bothan_binance::WorkerOpts> for AssetWorkerOpts {
    fn from(value: bothan_binance::WorkerOpts) -> Self {
        AssetWorkerOpts::Binance(value)
    }
}

impl From<bothan_bitfinex::WorkerOpts> for AssetWorkerOpts {
    fn from(value: bothan_bitfinex::WorkerOpts) -> Self {
        AssetWorkerOpts::Bitfinex(value)
    }
}

impl From<bothan_bybit::WorkerOpts> for AssetWorkerOpts {
    fn from(value: bothan_bybit::WorkerOpts) -> Self {
        AssetWorkerOpts::Bybit(value)
    }
}

impl From<bothan_coinbase::WorkerOpts> for AssetWorkerOpts {
    fn from(value: bothan_coinbase::WorkerOpts) -> Self {
        AssetWorkerOpts::Coinbase(value)
    }
}

impl From<bothan_coingecko::WorkerOpts> for AssetWorkerOpts {
    fn from(value: bothan_coingecko::WorkerOpts) -> Self {
        AssetWorkerOpts::CoinGecko(value)
    }
}

impl From<bothan_coinmarketcap::WorkerOpts> for AssetWorkerOpts {
    fn from(value: bothan_coinmarketcap::WorkerOpts) -> Self {
        AssetWorkerOpts::CoinMarketCap(value)
    }
}

impl From<bothan_htx::WorkerOpts> for AssetWorkerOpts {
    fn from(value: bothan_htx::WorkerOpts) -> Self {
        AssetWorkerOpts::Htx(value)
    }
}

impl From<bothan_kraken::WorkerOpts> for AssetWorkerOpts {
    fn from(value: bothan_kraken::WorkerOpts) -> Self {
        AssetWorkerOpts::Kraken(value)
    }
}

impl From<bothan_okx::WorkerOpts> for AssetWorkerOpts {
    fn from(value: bothan_okx::WorkerOpts) -> Self {
        AssetWorkerOpts::Okx(value)
    }
}

impl From<bothan_band::WorkerOpts> for AssetWorkerOpts {
    fn from(value: bothan_band::WorkerOpts) -> Self {
        AssetWorkerOpts::Band(value)
    }
}
