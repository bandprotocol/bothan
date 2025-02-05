pub mod error;
mod key;

use crate::store::rocks_db::error::{LoadError, RocksDbError};
use crate::store::rocks_db::key::Key;
use bincode::{config, decode_from_slice, encode_to_vec};
use bothan_lib::registry::{Registry, Valid};
use bothan_lib::store::Store;
use bothan_lib::types::AssetInfo;
use rust_rocksdb::{Options, WriteBatch, DB};
use std::collections::HashSet;
use std::hash::RandomState;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct RocksDbStore {
    db: Arc<DB>,
    registry: Arc<RwLock<Registry<Valid>>>,
}

impl RocksDbStore {
    pub fn new(registry: Registry<Valid>, flush_path: &str) -> Result<Self, rust_rocksdb::Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        DB::destroy(&opts, flush_path)?;

        let db = Arc::new(DB::open(&opts, flush_path)?);

        Ok(RocksDbStore {
            db,
            registry: Arc::new(RwLock::new(registry)),
        })
    }

    pub fn load(path: &str) -> Result<Self, LoadError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = Arc::new(DB::open(&opts, path)?);
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
        let hash = self
            .db
            .get(Key::RegistryIpfsHash.to_prefixed_bytes())?
            .map(|b| decode_from_slice(&b, config::standard()))
            .transpose()?
            .map(|(hash, _)| hash);
        Ok(hash)
    }

    async fn set_query_ids(&self, source_id: &str, ids: Vec<String>) -> Result<(), Self::Error> {
        let key = Key::QueryIDs { source_id };
        let set: HashSet<_, RandomState> = HashSet::from_iter(ids.into_iter());
        let encoded = encode_to_vec(&set, config::standard())?;
        self.db.put(key.to_prefixed_bytes(), encoded)?;
        Ok(())
    }

    async fn get_query_ids(&self, source_id: &str) -> Result<Option<Vec<String>>, Self::Error> {
        let key = Key::QueryIDs { source_id };

        let query_ids = self
            .db
            .get(key.to_prefixed_bytes())?
            .map(|b| decode_from_slice::<HashSet<String, RandomState>, _>(&b, config::standard()))
            .transpose()?
            .map(|(ids, _)| ids.into_iter().collect());
        Ok(query_ids)
    }

    async fn insert_query_ids(&self, source_id: &str, ids: Vec<String>) -> Result<(), Self::Error> {
        let key = Key::QueryIDs { source_id };

        let mut query_ids = self
            .db
            .get(key.to_prefixed_bytes())?
            .map(|b| decode_from_slice::<HashSet<String, RandomState>, _>(&b, config::standard()))
            .transpose()?
            .map(|(ids, _)| ids)
            .unwrap_or_default();

        query_ids.extend(ids.into_iter());
        let encoded = encode_to_vec(&query_ids, config::standard())?;
        self.db.put(key.to_prefixed_bytes(), encoded)?;
        Ok(())
    }

    async fn remove_query_ids(&self, source_id: &str, ids: &[String]) -> Result<(), Self::Error> {
        let key = Key::QueryIDs { source_id };

        let mut query_ids = self
            .db
            .get(key.to_prefixed_bytes())?
            .map(|b| decode_from_slice::<HashSet<String, RandomState>, _>(&b, config::standard()))
            .transpose()?
            .map(|(ids, _)| ids)
            .unwrap_or_default();

        for id in ids.iter() {
            query_ids.remove(id);
        }

        let encoded = encode_to_vec(&query_ids, config::standard())?;
        self.db.put(key.to_prefixed_bytes(), encoded)?;
        Ok(())
    }

    async fn contains_query_id(&self, source_id: &str, id: &str) -> Result<bool, Self::Error> {
        let key = Key::QueryIDs { source_id };

        let query_ids = self
            .db
            .get(key.to_prefixed_bytes())?
            .map(|b| decode_from_slice::<HashSet<String, RandomState>, _>(&b, config::standard()))
            .transpose()?
            .map(|(ids, _)| ids)
            .unwrap_or_default();

        Ok(query_ids.contains(id))
    }

    async fn get_asset_info(
        &self,
        source_id: &str,
        id: &str,
    ) -> Result<Option<AssetInfo>, Self::Error> {
        let key = Key::AssetStore { source_id, id };

        let encoded = self.db.get(key.to_prefixed_bytes())?;
        let asset_info = encoded
            .map(|b| decode_from_slice(&b, config::standard()))
            .transpose()?
            .map(|(info, _)| info);
        Ok(asset_info)
    }

    async fn insert_asset_info(
        &self,
        source_id: &str,
        asset_info: (String, AssetInfo),
    ) -> Result<(), Self::Error> {
        let key = Key::AssetStore {
            source_id,
            id: &asset_info.0,
        };

        let encoded = encode_to_vec(&asset_info, config::standard())?;
        self.db.put(key.to_prefixed_bytes(), encoded)?;
        Ok(())
    }

    async fn insert_asset_info_batch(
        &self,
        source_id: &str,
        asset_infos: Vec<(String, AssetInfo)>,
    ) -> Result<(), Self::Error> {
        let mut write_batch = WriteBatch::default();
        for (id, asset_info) in asset_infos {
            let key = Key::AssetStore { source_id, id: &id };
            let encoded = encode_to_vec(&asset_info, config::standard())?;
            write_batch.put(key.to_prefixed_bytes(), encoded);
        }

        self.db.write(write_batch)?;
        Ok(())
    }
}
