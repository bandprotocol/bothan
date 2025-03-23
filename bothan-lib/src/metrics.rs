pub mod rest;
pub mod server;
pub mod websocket;

pub struct Metrics {
    pub rest: rest::RestMetrics,
    pub server: server::ServerMetrics,
    pub websocket: websocket::WebSocketMetrics,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        let rest = rest::RestMetrics::new();
        let server = server::ServerMetrics::new();
        let websocket = websocket::WebSocketMetrics::new();

        Self {
            rest,
            server,
            websocket,
        }
    }
}
