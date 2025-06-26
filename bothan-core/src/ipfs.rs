//! IPFS integration for the Bothan core library.
//!
//! Provides IPFS client and builder types.

pub use builder::IpfsClientBuilder;
pub use client::IpfsClient;

mod builder;
mod client;
pub mod error;
