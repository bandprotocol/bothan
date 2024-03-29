use std::fs::File;
use std::sync::Arc;

use tonic::transport::Server;

use crate::api::APIServiceImpl;
use crate::manager::price_service::manager::PriceServiceManager;
use crate::proto::query::query::query_server::QueryServer;
use crate::registry::Registry;

mod api;
mod config;
mod manager;
mod post_processor;
mod processor;
mod proto;
mod registry;
mod service_builder;
mod tasks;
mod utils;

#[tokio::main]
async fn main() {
    let config = config::AppConfig::new().expect("Failed to load configuration");

    let file = File::open(config.registry.source.clone()).unwrap();
    let registry = Arc::new(serde_json::from_reader::<_, Registry>(file).unwrap());
    let mut manager = PriceServiceManager::new(registry)
        .expect("Cannot build Price Service Manager with Registry");
    let addr = config.grpc.addr.clone().parse().unwrap();

    service_builder::build_services(config, &mut manager).await;
    let api_service_impl = APIServiceImpl::new(manager);
    println!("Server running on {}", addr);

    let _ = Server::builder()
        .add_service(QueryServer::new(api_service_impl))
        .serve(addr)
        .await;
}
