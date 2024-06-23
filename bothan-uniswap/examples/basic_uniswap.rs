use tracing_subscriber::fmt::init;

use bothan_core::service::Service;
use bothan_uniswap::service::v3::builder::UniswapV3ServiceBuilder;

#[tokio::main]
async fn main() {
    init();
    let service = UniswapV3ServiceBuilder::default()
        .with_provider("https://eth.llamarpc.com/")
        .build_with_reqwest_http()
        .await;

    if let Ok(mut service) = service {
        loop {
            let data = service
                .get_price_data(&["0x4e68Ccd3E89f51C3074ca5072bbAC773960dFa36"])
                .await;
            println!("{:?}", data);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
