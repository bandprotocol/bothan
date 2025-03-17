use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Extension, Router};
use axum::response::IntoResponse;
use axum::routing::get;

use prometheus::{Encoder, TextEncoder};

use crate::telemetry::state::TelemetryState;

pub async fn listen(
    addr: SocketAddr,
    state: Arc<TelemetryState>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn get_metrics(
    Extension(state): Extension<Arc<TelemetryState>>,
) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&state.gather(), &mut buffer).unwrap();

    ([("content-type", "text/plain; charset=utf-8")], buffer)
}
