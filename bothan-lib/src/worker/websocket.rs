use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

use tokio::select;
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::store::{Store, WorkerStore};
use crate::types::AssetInfo;

pub enum Data {
    AssetInfo(Vec<AssetInfo>),
    Unused,
}

#[async_trait::async_trait]
pub trait AssetInfoProviderConnector: Send + Sync {
    type Provider: AssetInfoProvider;
    type Error: Display;

    async fn connect(&self) -> Result<Self::Provider, Self::Error>;
}

#[async_trait::async_trait]
pub trait AssetInfoProvider: Send + Sync {
    type SubscriptionError: Display;
    type ListeningError: Display;

    async fn subscribe(&mut self, ids: &[String]) -> Result<(), Self::SubscriptionError>;

    async fn next(&mut self) -> Option<Result<Data, Self::ListeningError>>;

    async fn try_close(mut self);
}

#[tracing::instrument(skip(
    cancellation_token,
    provider_connector,
    store,
    ids,
    connection_timeout
))]
pub async fn start_listening<S, E1, E2, P, C>(
    cancellation_token: CancellationToken,
    provider_connector: Arc<C>,
    store: WorkerStore<S>,
    ids: Vec<String>,
    connection_timeout: Duration,
) where
    E1: Display,
    E2: Display,
    S: Store,
    P: AssetInfoProvider<SubscriptionError = E1, ListeningError = E2>,
    C: AssetInfoProviderConnector<Provider = P>,
{
    let mut connection = connect(provider_connector.as_ref(), &ids).await;
    loop {
        select! {
            _ = cancellation_token.cancelled() => break,
            result = timeout(connection_timeout, connection.next()) => {
                    match result {
                        // If timeout, we assume the connection has been dropped, and we attempt to reconnect
                        Err(_) | Ok(None) => {
                            let new_conn = connect(provider_connector.as_ref(), &ids).await;
                            connection = new_conn
                        }
                        Ok(Some(Ok(Data::AssetInfo(assets)))) => {
                            if let Err(e) = store.set_batch_asset_info(assets).await {
                                error!("failed to set asset info with error: {e}")
                            } else {
                                info!("asset info updated successfully");
                            }
                        }
                        Ok(Some(Ok(Data::Unused))) => debug!("received irrelevant data"),
                        Ok(Some(Err(e))) => error!("{}", e),
                    }
            }
        }
    }
}

async fn connect<C, P, E1, E2>(connector: &C, ids: &[String]) -> P
where
    P: AssetInfoProvider<SubscriptionError = E1, ListeningError = E2>,
    C: AssetInfoProviderConnector<Provider = P>,
{
    let mut retry_count = 0;
    let mut backoff = Duration::from_secs(1);
    let max_backoff = Duration::from_secs(64);

    loop {
        if let Ok(mut provider) = connector.connect().await {
            if provider.subscribe(ids).await.is_ok() {
                return provider;
            }
        }

        retry_count += 1;
        if backoff < max_backoff {
            backoff *= 2;
        }

        error!("failed to reconnect. current attempt: {}", retry_count);
        sleep(backoff).await;
    }
}
