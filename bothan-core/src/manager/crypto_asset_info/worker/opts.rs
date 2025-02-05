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

impl Into<CryptoAssetWorkerOpts> for bothan_binance::WorkerOpts {
    fn into(self) -> CryptoAssetWorkerOpts {
        CryptoAssetWorkerOpts::Binance(self)
    }
}

impl Into<CryptoAssetWorkerOpts> for bothan_bitfinex::WorkerOpts {
    fn into(self) -> CryptoAssetWorkerOpts {
        CryptoAssetWorkerOpts::Bitfinex(self)
    }
}

impl Into<CryptoAssetWorkerOpts> for bothan_bybit::WorkerOpts {
    fn into(self) -> CryptoAssetWorkerOpts {
        CryptoAssetWorkerOpts::Bybit(self)
    }
}

impl Into<CryptoAssetWorkerOpts> for bothan_coinbase::WorkerOpts {
    fn into(self) -> CryptoAssetWorkerOpts {
        CryptoAssetWorkerOpts::Coinbase(self)
    }
}

impl Into<CryptoAssetWorkerOpts> for bothan_coingecko::WorkerOpts {
    fn into(self) -> CryptoAssetWorkerOpts {
        CryptoAssetWorkerOpts::CoinGecko(self)
    }
}

impl Into<CryptoAssetWorkerOpts> for bothan_coinmarketcap::WorkerOpts {
    fn into(self) -> CryptoAssetWorkerOpts {
        CryptoAssetWorkerOpts::CoinMarketCap(self)
    }
}

impl Into<CryptoAssetWorkerOpts> for bothan_htx::WorkerOpts {
    fn into(self) -> CryptoAssetWorkerOpts {
        CryptoAssetWorkerOpts::Htx(self)
    }
}

impl Into<CryptoAssetWorkerOpts> for bothan_kraken::WorkerOpts {
    fn into(self) -> CryptoAssetWorkerOpts {
        CryptoAssetWorkerOpts::Kraken(self)
    }
}

impl Into<CryptoAssetWorkerOpts> for bothan_okx::WorkerOpts {
    fn into(self) -> CryptoAssetWorkerOpts {
        CryptoAssetWorkerOpts::Okx(self)
    }
}
