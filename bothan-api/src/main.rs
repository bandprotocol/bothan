mod config;

mod post_processor;
mod processor;
mod proto;
mod task;

#[tokio::main]
async fn main() {
    let _ = config::AppConfig::new().unwrap();
    todo!()
}
