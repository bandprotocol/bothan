use std::sync::Arc;
use tokio::sync::mpsc::{channel, Sender};

use crate::api::websocket::HtxWebSocketConnector;
use crate::worker::asset_worker::start_asset_worker;
use crate::WorkerOpts;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::types::AssetState;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::AssetWorker;

mod asset_worker;
pub mod opts;
mod types;

const WORKER_NAME: &str = "htx";

pub struct Worker<S: Store> {
    inner: Arc<InnerWorker<S>>,
}

#[async_trait::async_trait]
impl<S: Store + 'static> AssetWorker<S> for Worker<S> {
    type Opts = WorkerOpts;

    fn name(&self) -> &'static str {
        WORKER_NAME
    }

    async fn build(opts: Self::Opts, store: &S) -> Result<Self, AssetWorkerError> {
        let url = opts.url;
        let ch_size = opts.internal_ch_size;

        let connector = HtxWebSocketConnector::new(url);
        let connection = connector.connect().await?;

        let (sub_tx, sub_rx) = channel(ch_size);
        let (unsub_tx, unsub_rx) = channel(ch_size);

        let worker_store = WorkerStore::new(store, WORKER_NAME);
        let to_sub = worker_store
            .get_query_ids()
            .await?
            .into_iter()
            .collect::<Vec<String>>();

        if !to_sub.is_empty() {
            sub_tx.send(to_sub).await?;
        }

        let inner = Arc::new(InnerWorker {
            connector,
            store: worker_store,
            subscribe_tx: sub_tx,
            unsubscribe_tx: unsub_tx,
        });

        tokio::spawn(start_asset_worker(
            Arc::downgrade(&inner),
            connection,
            sub_rx,
            unsub_rx,
        ));

        Ok(Worker { inner })
    }

    async fn get_asset(&self, id: &str) -> Result<AssetState, AssetWorkerError> {
        Ok(self.inner.store.get_asset(id).await?)
    }

    async fn set_query_ids(&self, ids: Vec<String>) -> Result<(), AssetWorkerError> {
        let diff = self.inner.store.compute_query_id_difference(ids).await?;

        self.inner.subscribe_tx.send(diff.added).await?;
        self.inner.unsubscribe_tx.send(diff.removed).await?;

        Ok(())
    }
}

struct InnerWorker<S: Store> {
    connector: HtxWebSocketConnector,
    store: WorkerStore<S>,
    subscribe_tx: Sender<Vec<String>>,
    unsubscribe_tx: Sender<Vec<String>>,
}
