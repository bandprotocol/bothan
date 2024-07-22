use serde::Deserialize;

/// The configuration for bothan-api's gRPC server.
#[derive(Clone, Debug, Deserialize)]
pub struct GrpcConfig {
    /// The address to bind the gRPC server too.
    pub addr: String,
}
