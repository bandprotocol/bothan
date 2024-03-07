mod config;

mod post_processors;
mod processors;
mod proto;

#[tokio::main]
async fn main() {
    let _ = config::AppConfig::new().unwrap();
    todo!()
}
