use bothan_binance::BinanceService;
use bothan_coingecko::CoinGeckoService;
use bothan_core::service::{Service as CoreService, ServiceResult};
use bothan_core::types::PriceData;

pub enum Service {
    Binance(BinanceService),
    CoinGecko(CoinGeckoService),
}

impl Service {
    pub async fn get_price_data(&mut self, ids: &[&str]) -> Vec<ServiceResult<PriceData>> {
        match self {
            Service::Binance(service) => service.get_price_data(ids).await,
            Service::CoinGecko(service) => service.get_price_data(ids).await,
        }
    }
}
