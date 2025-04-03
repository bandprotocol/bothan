use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

use tokio::select;
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, warn};

use crate::metrics::websocket::{ConnectionResult, MessageType, Metrics};
use crate::store::{Store, WorkerStore};
use crate::types::AssetInfo;

pub enum Data {
    AssetInfo(Vec<AssetInfo>),
    Ping,
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
    type PollingError: Display;

    async fn subscribe(&mut self, ids: &[String]) -> Result<(), Self::SubscriptionError>;

    async fn next(&mut self) -> Option<Result<Data, Self::PollingError>>;

    async fn try_close(mut self);
}

#[tracing::instrument(skip(
    cancellation_token,
    provider_connector,
    store,
    ids,
    connection_timeout
))]
pub async fn start_polling<S, E1, E2, P, C>(
    cancellation_token: CancellationToken,
    provider_connector: Arc<C>,
    store: WorkerStore<S>,
    ids: Vec<String>,
    connection_timeout: Duration,
    metrics: Metrics,
) where
    E1: Display,
    E2: Display,
    S: Store,
    P: AssetInfoProvider<SubscriptionError = E1, PollingError = E2>,
    C: AssetInfoProviderConnector<Provider = P>,
{
    let mut connection = connect(provider_connector.as_ref(), &ids, &metrics).await;
    loop {
        select! {
            _ = cancellation_token.cancelled() => break,
            poll_result = timeout(connection_timeout, connection.next()) => {
                    match poll_result {
                        // If timeout, we assume the connection has been dropped, and we attempt to reconnect
                        Err(_) | Ok(None) => {
                            let new_conn = connect(provider_connector.as_ref(), &ids, &metrics).await;
                            connection = new_conn
                        }
                        Ok(Some(Ok(Data::AssetInfo(ai)))) => {
                            if let Err(e) = store.set_batch_asset_info(ai).await {
                                warn!("failed to store data: {}", e)
                            }
                            metrics.increment_activity_messages_total(MessageType::AssetInfo)
                        }
                        Ok(Some(Ok(Data::Ping))) => metrics.increment_activity_messages_total(MessageType::Ping),
                        Ok(Some(Ok(Data::Unused))) => debug!("received irrelevant data"),
                        Ok(Some(Err(e))) => error!("{}", e),
                    }
            }
        }
    }

    debug!("asset worker has been dropped, stopping asset worker");
}

// If connect fails, the polling loop should be exited
async fn connect<C, P, E1, E2>(connector: &C, ids: &[String], metrics: &Metrics) -> P
where
    P: AssetInfoProvider<SubscriptionError = E1, PollingError = E2>,
    C: AssetInfoProviderConnector<Provider = P>,
{
    let mut retry_count = 0;
    let mut backoff = Duration::from_secs(1);
    let max_backoff = Duration::from_secs(516);

    loop {
        let start_time = chrono::Utc::now().timestamp_millis();

        if let Ok(mut provider) = connector.connect().await {
            if provider.subscribe(ids).await.is_ok() {
                let _ = metrics.record_connection_duration(
                    chrono::Utc::now().timestamp_millis() - start_time,
                    ConnectionResult::Success,
                );
                metrics.increment_connections_total(ConnectionResult::Success);
                return provider;
            }
        }

        metrics.record_failed_connection_retry_count(retry_count);

        retry_count += 1;
        if backoff < max_backoff {
            // Set max backoff to 516 seconds
            backoff *= 2;
        }

        let _ = metrics.record_connection_duration(
            chrono::Utc::now().timestamp_millis() - start_time,
            ConnectionResult::Failed,
        );
        metrics.increment_connections_total(ConnectionResult::Failed);
        error!("failed to reconnect. current attempt: {}", retry_count);
        sleep(backoff).await;
    }
}
