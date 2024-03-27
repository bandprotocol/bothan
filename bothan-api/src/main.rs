use std::fs::File;
use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

use bothan_binance::BinanceServiceBuilder;
use bothan_coingecko::CoinGeckoServiceBuilder;
use bothan_coinmarketcap::CoinMarketCapServiceBuilder;
use bothan_cryptocompare::CryptoCompareServiceBuilder;
use bothan_htx::HtxServiceBuilder;

use crate::manager::price_service::manager::PriceServiceManager;
use crate::proto::query::query::query_server::{Query, QueryServer};
use crate::proto::query::query::{QueryPricesRequest, QueryPricesResponse};
use crate::registry::Registry;
use crate::utils::arc_mutex;

mod config;
mod manager;
mod post_processor;
mod processor;
mod proto;
mod registry;
mod tasks;
mod utils;

pub struct PriceServiceImpl {
    manager: Arc<Mutex<PriceServiceManager>>,
}

impl PriceServiceImpl {
    fn new(manager: PriceServiceManager) -> Self {
        PriceServiceImpl {
            manager: arc_mutex!(manager),
        }
    }
}

#[tonic::async_trait]
impl Query for PriceServiceImpl {
    async fn prices(
        &self, // Change to accept mutable reference
        request: Request<QueryPricesRequest>,
    ) -> Result<Response<QueryPricesResponse>, Status> {
        let signal_ids = request.into_inner().signal_ids;
        println!("Received signal_ids: {:?}", signal_ids);
        let l = &signal_ids
            .iter()
            .map(|symbol| symbol.as_str())
            .collect::<Vec<&str>>();

        let mut manager = self.manager.lock().await;
        let prices = manager.get_prices(l).await;

        let response = QueryPricesResponse { prices };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() {
    let config = config::AppConfig::new().expect("Failed to load configuration");

    let file = File::open(config.registry.source).unwrap();
    let registry = Arc::new(serde_json::from_reader::<_, Registry>(file).unwrap());
    let mut manager = PriceServiceManager::new(registry);

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

    let price_service_impl = PriceServiceImpl::new(manager);
    let addr = config.grpc.addr.parse().unwrap();
    println!("Server running on {}", addr);

    let _ = Server::builder()
        .add_service(QueryServer::new(price_service_impl))
        .serve(addr)
        .await;
}
