pub mod error;
mod key;

use crate::store::rocks_db::error::{LoadError, RocksDbError};
use crate::store::rocks_db::key::Key;
use bincode::{config, decode_from_slice, encode_to_vec, Decode, Encode};
use bothan_lib::registry::{Registry, Valid};
use bothan_lib::store::Store;
use bothan_lib::types::AssetInfo;
use rust_rocksdb::{Options, WriteBatch, DB};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct RocksDbStore {
    db: Arc<DB>,
    registry: Arc<RwLock<Registry<Valid>>>,
}

impl RocksDbStore {
    pub fn new(flush_path: &str) -> Result<Self, rust_rocksdb::Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        DB::destroy(&opts, flush_path)?;

        let db = Arc::new(DB::open(&opts, flush_path)?);
        Ok(RocksDbStore {
            db,
            registry: Arc::new(RwLock::new(Registry::default())),
        })
    }

    pub fn load(flush_path: &str) -> Result<Self, LoadError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = Arc::new(DB::open(&opts, flush_path)?);
        let unvalidated_registry = db
            .get(Key::Registry.to_prefixed_bytes())?
            .map(|b| decode_from_slice::<Registry, _>(&b, config::standard()))
            .transpose()?
            .map(|(r, _)| r)
            .ok_or(LoadError::NoExistingRegistry)?;

        let registry = Registry::validate(unvalidated_registry)?;

        Ok(RocksDbStore {
            db,
            registry: Arc::new(RwLock::new(registry)),
        })
    }

    fn set<V: Encode>(&self, key: &Key, value: &V) -> Result<(), RocksDbError> {
        let encoded = encode_to_vec(value, config::standard())?;
        self.db.put(key.to_prefixed_bytes(), encoded)?;
        Ok(())
    }

    fn get<V: Decode>(&self, key: &Key) -> Result<Option<V>, RocksDbError> {
        let value = self
            .db
            .get(key.to_prefixed_bytes())?
            .map(|b| decode_from_slice(&b, config::standard()))
            .transpose()?
            .map(|(v, _)| v);
        Ok(value)
    }
}

#[async_trait::async_trait]
impl Store for RocksDbStore {
    type Error = RocksDbError;

    async fn set_registry(
        &self,
        registry: Registry<Valid>,
        ipfs_hash: String,
    ) -> Result<(), Self::Error> {
        let encoded_registry = encode_to_vec(&registry, config::standard())?;
        let encoded_hash = encode_to_vec(&ipfs_hash, config::standard())?;

        // if the registry can be encoded, lock first to prevent race conditions
        let mut curr_reg = self.registry.write().await;

        // save to db
        let mut write_batch = WriteBatch::default();
        write_batch.put(Key::Registry.to_prefixed_bytes(), encoded_registry);
        write_batch.put(Key::RegistryIpfsHash.to_prefixed_bytes(), encoded_hash);

        self.db.write(write_batch)?;

        // save to local
        *curr_reg = registry;

        Ok(())
    }

    async fn get_registry(&self) -> Registry<Valid> {
        self.registry.read().await.clone()
    }

    async fn get_registry_ipfs_hash(&self) -> Result<Option<String>, Self::Error> {
        self.get(&Key::RegistryIpfsHash)
    }

    async fn set_query_ids(&self, prefix: &str, ids: HashSet<String>) -> Result<(), Self::Error> {
        self.set(&Key::QueryIDs { source_id: prefix }, &ids)
    }

    async fn get_query_ids(&self, prefix: &str) -> Result<Option<HashSet<String>>, Self::Error> {
        self.get(&Key::QueryIDs { source_id: prefix })
    }

    async fn contains_query_id(&self, prefix: &str, id: &str) -> Result<bool, Self::Error> {
        let bool = self
            .get::<HashSet<String>>(&Key::QueryIDs { source_id: prefix })?
            .unwrap_or_default()
            .contains(id);
        Ok(bool)
    }

    async fn get_asset_info(
        &self,
        prefix: &str,
        id: &str,
    ) -> Result<Option<AssetInfo>, Self::Error> {
        self.get(&Key::AssetStore {
            source_id: prefix,
            id,
        })
    }

    async fn insert_asset_info(
        &self,
        prefix: &str,
        asset_info: AssetInfo,
    ) -> Result<(), Self::Error> {
        let key = Key::AssetStore {
            source_id: prefix,
            id: &asset_info.id,
        };
        self.set(&key, &asset_info)
    }

    async fn insert_asset_infos(
        &self,
        prefix: &str,
        asset_infos: Vec<AssetInfo>,
    ) -> Result<(), Self::Error> {
        let mut write_batch = WriteBatch::default();
        for asset_info in asset_infos {
            let key = Key::AssetStore {
                source_id: prefix,
                id: &asset_info.id,
            };
            let encoded = encode_to_vec(&asset_info, config::standard())?;
            write_batch.put(key.to_prefixed_bytes(), encoded);
        }

        self.db.write(write_batch)?;
        Ok(())
    }
}
