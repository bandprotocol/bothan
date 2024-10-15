pub use websocket::{
    CryptoCompareWebSocketConnection, CryptoCompareWebSocketConnector, DEFAULT_URL,
};

pub mod errors;
pub mod msgs;
mod websocket;
