use serde::Deserialize;

/// The configuration for the gRPC server.
#[derive(Clone, Debug, Deserialize)]
pub struct GrpcConfig {
    pub addr: String,
}
