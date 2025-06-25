//! Bothan core RocksDB store key types.
//!
//! Provides key types and utilities for RocksDB storage.

use std::fmt::Display;

pub enum Key<'a> {
    AssetStore {
        source_id: &'a str,
        asset_id: &'a str,
    },
    Registry,
    RegistryIpfsHash,
}

/// Key type for RocksDB store.
impl Display for Key<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Key::AssetStore {
                source_id,
                asset_id,
            } => format!("asset_store::{}::{}", source_id, asset_id),
            Key::Registry => "registry".to_string(),
            Key::RegistryIpfsHash => "registry_ipfs_hash".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl Key<'_> {
    pub fn to_prefixed_bytes(&self) -> Vec<u8> {
        let prefix = "bothan::".as_bytes();
        let content = self.to_string().into_bytes();
        [prefix, &content].concat()
    }
}
