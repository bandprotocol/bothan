#[derive(Clone)]
pub enum CryptoAssetWorkerOpts {
    Binance(bothan_binance::WorkerOpts),
    Bitfinex(bothan_bitfinex::WorkerOpts),
    Bybit(bothan_bybit::WorkerOpts),
    Coinbase(bothan_coinbase::WorkerOpts),
    CoinGecko(bothan_coingecko::WorkerOpts),
    CoinMarketCap(bothan_coinmarketcap::WorkerOpts),
    Htx(bothan_htx::WorkerOpts),
    Kraken(bothan_kraken::WorkerOpts),
    Okx(bothan_okx::WorkerOpts),
}

impl CryptoAssetWorkerOpts {
    pub fn name(&self) -> &str {
        match self {
            CryptoAssetWorkerOpts::Binance(_) => "binance",
            CryptoAssetWorkerOpts::Bitfinex(_) => "bitfinex",
            CryptoAssetWorkerOpts::Bybit(_) => "bybit",
            CryptoAssetWorkerOpts::Coinbase(_) => "coinbase",
            CryptoAssetWorkerOpts::CoinGecko(_) => "coingecko",
            CryptoAssetWorkerOpts::CoinMarketCap(_) => "coinmarketcap",
            CryptoAssetWorkerOpts::Htx(_) => "htx",
            CryptoAssetWorkerOpts::Kraken(_) => "kraken",
            CryptoAssetWorkerOpts::Okx(_) => "okx",
        }
    }
}

impl From<bothan_binance::WorkerOpts> for CryptoAssetWorkerOpts {
    fn from(value: bothan_binance::WorkerOpts) -> Self {
        CryptoAssetWorkerOpts::Binance(value)
    }
}

impl From<bothan_bitfinex::WorkerOpts> for CryptoAssetWorkerOpts {
    fn from(value: bothan_bitfinex::WorkerOpts) -> Self {
        CryptoAssetWorkerOpts::Bitfinex(value)
    }
}

impl From<bothan_bybit::WorkerOpts> for CryptoAssetWorkerOpts {
    fn from(value: bothan_bybit::WorkerOpts) -> Self {
        CryptoAssetWorkerOpts::Bybit(value)
    }
}

impl From<bothan_coinbase::WorkerOpts> for CryptoAssetWorkerOpts {
    fn from(value: bothan_coinbase::WorkerOpts) -> Self {
        CryptoAssetWorkerOpts::Coinbase(value)
    }
}

impl From<bothan_coingecko::WorkerOpts> for CryptoAssetWorkerOpts {
    fn from(value: bothan_coingecko::WorkerOpts) -> Self {
        CryptoAssetWorkerOpts::CoinGecko(value)
    }
}

impl From<bothan_coinmarketcap::WorkerOpts> for CryptoAssetWorkerOpts {
    fn from(value: bothan_coinmarketcap::WorkerOpts) -> Self {
        CryptoAssetWorkerOpts::CoinMarketCap(value)
    }
}

impl From<bothan_htx::WorkerOpts> for CryptoAssetWorkerOpts {
    fn from(value: bothan_htx::WorkerOpts) -> Self {
        CryptoAssetWorkerOpts::Htx(value)
    }
}

impl From<bothan_kraken::WorkerOpts> for CryptoAssetWorkerOpts {
    fn from(value: bothan_kraken::WorkerOpts) -> Self {
        CryptoAssetWorkerOpts::Kraken(value)
    }
}

impl From<bothan_okx::WorkerOpts> for CryptoAssetWorkerOpts {
    fn from(value: bothan_okx::WorkerOpts) -> Self {
        CryptoAssetWorkerOpts::Okx(value)
    }
}
