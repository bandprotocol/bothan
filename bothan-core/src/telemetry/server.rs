use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;

use axum::{Extension, Router};
use axum::response::IntoResponse;
use axum::routing::get;

use prometheus::{Encoder, Registry, TextEncoder};
use tokio::task::JoinHandle;

pub type BoxError = Box<dyn Error + Send + Sync>;

pub fn spawn_server<A>(
    addr: A,
    registry: Arc<Registry>,
) -> Result<(SocketAddr, JoinHandle<Result<(), BoxError>>), BoxError>
where
    A: ToSocketAddrs + Send + 'static,
{
    let addr = addr.to_socket_addrs()?.next().unwrap();
    let handle = tokio::spawn(listen(addr, registry));

    Ok((addr, handle))
}

async fn listen(
    addr: SocketAddr,
    state: Arc<Registry>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn get_metrics(
    Extension(state): Extension<Arc<Registry>>,
) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&state.gather(), &mut buffer).unwrap();

    ([("content-type", "text/plain; charset=utf-8")], buffer)
}
