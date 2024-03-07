mod config;

mod post_processor;
mod processor;
mod proto;
mod tasks;

#[tokio::main]
async fn main() {
    let _ = config::AppConfig::new().unwrap();
    todo!()
}
