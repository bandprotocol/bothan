mod config;
mod post_processor;
mod processor;
mod proto;

#[tokio::main]
async fn main() {
    let _ = config::AppConfig::new().unwrap();
    todo!()
}
