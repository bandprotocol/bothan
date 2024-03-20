use std::fs::File;
use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

use bothan_binance::BinanceServiceBuilder;
use bothan_coingecko::CoinGeckoServiceBuilder;

use crate::manager::price_service::manager::PriceServiceManager;
use crate::proto::query::query::query_server::{Query, QueryServer};
use crate::proto::query::query::{QueryPricesRequest, QueryPricesResponse};
use crate::registry::Registry;
use crate::util::arc_mutex;
use bothan_core::service::Service as CoreService;

mod config;
mod manager;
mod post_processor;
mod processor;
mod proto;
mod registry;
mod tasks;
mod util;

pub struct PriceServiceImpl<T: CoreService> {
    manager: Arc<Mutex<PriceServiceManager<T>>>,
}

impl<T: CoreService> PriceServiceImpl<T> {
    fn new(manager: PriceServiceManager<T>) -> Self {
        PriceServiceImpl {
            manager: arc_mutex!(manager),
        }
    }
}

#[tonic::async_trait]
impl<T: CoreService> Query for PriceServiceImpl<T> {
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
    manager.add_service("binance".to_string(), binance).await;

    let coingecko = CoinGeckoServiceBuilder::new(config.source.coingecko)
        .build()
        .await
        .unwrap();
    manager
        .add_service("coingecko".to_string(), coingecko)
        .await;

    let price_data_impl = PriceServiceImpl::new(manager);
    let addr = config.grpc.addr.parse().unwrap();
    println!("Server running on {}", addr);

    let _ = Server::builder()
        .add_service(QueryServer::new(price_data_impl))
        .serve(addr)
        .await;
}
