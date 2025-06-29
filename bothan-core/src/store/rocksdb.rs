//! Bothan core RocksDB store implementation.
//!
//! Provides RocksDB-backed storage for Bothan core, including serialization, deserialization, and batch operations for registry and asset information.

pub mod error;
mod key;

use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use bincode::{Decode, Encode, config, decode_from_slice, encode_to_vec};
use bothan_lib::metrics::store::{Metrics, Operation, OperationStatus};
use bothan_lib::registry::{Registry, Valid};
use bothan_lib::store::Store;
use bothan_lib::types::AssetInfo;
use rust_rocksdb::{DB, Options, WriteBatch};

use crate::store::rocksdb::error::{LoadError, RocksDbError};
use crate::store::rocksdb::key::Key;

#[derive(Clone)]
/// RocksDB-backed store for Bothan core.
pub struct RocksDbStore {
    db: Arc<DB>,
    registry: Arc<RwLock<Registry<Valid>>>,
    metrics: Metrics,
}

impl RocksDbStore {
    pub fn new<P: AsRef<Path>>(flush_path: P) -> Result<Self, RocksDbError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        DB::destroy(&opts, &flush_path)?;
        let registry = Registry::default();

        let encoded_registry = encode_to_vec(&registry, config::standard())?;
        let encoded_hash = encode_to_vec("", config::standard())?;

        let db = Arc::new(DB::open(&opts, &flush_path)?);

        let mut write_batch = WriteBatch::default();
        write_batch.put(Key::Registry.to_prefixed_bytes(), encoded_registry);
        write_batch.put(Key::RegistryIpfsHash.to_prefixed_bytes(), encoded_hash);

        db.write(write_batch)?;

        let store = RocksDbStore {
            db,
            registry: Arc::new(RwLock::new(registry)),
            metrics: Metrics::new(),
        };

        Ok(store)
    }

    pub fn load<P: AsRef<Path>>(flush_path: P) -> Result<Self, LoadError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = Arc::new(DB::open(&opts, &flush_path)?);
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
            metrics: Metrics::new(),
        })
    }

    fn set<V: Encode>(&self, key: &Key, value: &V) -> Result<(), RocksDbError> {
        let encoded = encode_to_vec(value, config::standard())?;
        self.db.put(key.to_prefixed_bytes(), encoded)?;
        Ok(())
    }

    fn get<V: Decode<()>>(&self, key: &Key) -> Result<Option<V>, RocksDbError> {
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
        let encoded_hash = encode_to_vec(ipfs_hash, config::standard())?;

        let mut writer = self
            .registry
            .write()
            .map_err(|_| RocksDbError::PoisonedError)?;

        let mut write_batch = WriteBatch::default();
        write_batch.put(Key::Registry.to_prefixed_bytes(), encoded_registry);
        write_batch.put(Key::RegistryIpfsHash.to_prefixed_bytes(), encoded_hash);

        self.db.write(write_batch)?;
        *writer = registry;
        Ok(())
    }

    async fn get_registry(&self) -> Result<Registry<Valid>, Self::Error> {
        Ok(self
            .registry
            .read()
            .map_err(|_| RocksDbError::PoisonedError)?
            .clone())
    }

    async fn get_registry_ipfs_hash(&self) -> Result<Option<String>, Self::Error> {
        self.get(&Key::RegistryIpfsHash)
    }

    async fn get_asset_info(
        &self,
        prefix: &str,
        id: &str,
    ) -> Result<Option<AssetInfo>, Self::Error> {
        self.get(&Key::AssetStore {
            source_id: prefix,
            asset_id: id,
        })
    }

    async fn insert_asset_info(
        &self,
        prefix: &str,
        asset_info: AssetInfo,
    ) -> Result<(), Self::Error> {
        let key = Key::AssetStore {
            source_id: prefix,
            asset_id: &asset_info.id,
        };
        self.set(&key, &asset_info)
    }

    async fn insert_batch_asset_info(
        &self,
        prefix: &str,
        asset_infos: Vec<AssetInfo>,
    ) -> Result<(), Self::Error> {
        let start_time = Instant::now();
        let mut write_batch = WriteBatch::default();
        for asset_info in asset_infos {
            let key = Key::AssetStore {
                source_id: prefix,
                asset_id: &asset_info.id,
            };
            let encoded = encode_to_vec(&asset_info, config::standard()).inspect_err(|_| {
                self.metrics.update_store_operation(
                    prefix.to_string(),
                    start_time.elapsed().as_micros(),
                    Operation::InsertBatchAssetInfo,
                    OperationStatus::Failed,
                );
            })?;
            write_batch.put(key.to_prefixed_bytes(), encoded);
        }

        self.db.write(write_batch).inspect_err(|_| {
            self.metrics.update_store_operation(
                prefix.to_string(),
                start_time.elapsed().as_micros(),
                Operation::InsertBatchAssetInfo,
                OperationStatus::Failed,
            );
        })?;

        self.metrics.update_store_operation(
            prefix.to_string(),
            start_time.elapsed().as_micros(),
            Operation::InsertBatchAssetInfo,
            OperationStatus::Success,
        );
        Ok(())
    }
}
