mod config;
mod proto;

#[tokio::main]
async fn main() {
    let _ = config::AppConfig::new().unwrap();
    todo!()
}
