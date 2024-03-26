mod config;
mod manager;
mod post_processor;
mod processor;
mod proto;
mod registry;
mod tasks;
mod utils;

#[tokio::main]
async fn main() {
    let _ = config::AppConfig::new().unwrap();
    todo!()
}
