mod config;
mod manager;
mod post_processor;
mod processor;
mod proto;
mod registry;
mod tasks;

#[tokio::main]
async fn main() {
    let _ = config::AppConfig::new().unwrap();
    todo!()
}
