use std::net::SocketAddr;

use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Extension, Router};
use hyper::body::Bytes;
use prometheus::{Encoder, Registry, TextEncoder};
use tokio::net::TcpListener;
use tracing::{error, info};

pub async fn spawn_server(addr: SocketAddr, registry: Registry) {
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .layer(Extension(registry));

    match TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("telemetry server listening on {}", addr);
            if let Err(e) = axum::serve(listener, app.into_make_service()).await {
                error!("failed to start telemetry server: {}", e);
            }
        }
        Err(e) => {
            error!("failed to bind telemetry server to {}: {}", addr, e);
        }
    }
}

async fn get_metrics(Extension(registry): Extension<Registry>) -> Response {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();

    match encoder.encode(&registry.gather(), &mut buffer) {
        Ok(_) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, encoder.format_type())],
            Bytes::from(buffer),
        )
            .into_response(),
        Err(e) => {
            error!("failed to encode metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, encoder.format_type())],
                "failed to encode metrics",
            )
                .into_response()
        }
    }
}
