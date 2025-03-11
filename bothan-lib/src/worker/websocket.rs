use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

use opentelemetry::{KeyValue, global};
use tokio::select;
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, warn};

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
    type PollingError: Display;

    async fn subscribe(&mut self, ids: &[String]) -> Result<(), Self::SubscriptionError>;

    async fn next(&mut self) -> Option<Result<Data, Self::PollingError>>;

    async fn try_close(mut self);
}

#[derive(Clone)]
pub struct PollOptions {
    pub timeout: Duration,
    pub reconnect_buffer: Duration,
    pub max_retry: u64,
    pub meter_name: &'static str,
}

// TODO: improve logging here
pub async fn start_polling<S, E1, E2, P, C>(
    cancellation_token: CancellationToken,
    provider_connector: Arc<C>,
    store: WorkerStore<S>,
    ids: Vec<String>,
    opts: PollOptions,
) where
    E1: Display,
    E2: Display,
    S: Store,
    P: AssetInfoProvider<SubscriptionError = E1, PollingError = E2>,
    C: AssetInfoProviderConnector<Provider = P>,
{
    if let Some(mut connection) = connect(provider_connector.as_ref(), &ids, &opts).await {
        loop {
            select! {
                _ = cancellation_token.cancelled() => break,
                poll_result = timeout(opts.timeout, connection.next()) => {
                        match poll_result {
                            // If timeout, we assume the connection has been dropped, and we attempt to reconnect
                            Err(_) | Ok(None) => {
                                if let Some(new_conn) = connect(provider_connector.as_ref(), &ids, &opts).await {
                                    connection = new_conn
                                } else {
                                    error!("failed to reconnect");
                                    break;
                                }
                            }
                            Ok(Some(Ok(Data::AssetInfo(ai)))) => {
                                if let Err(e) = store.set_batch_asset_info(ai).await {
                                    warn!("failed to store data: {}", e)
                                }
                            }
                            Ok(Some(Ok(Data::Unused))) => debug!("received irrelevant data"),
                            Ok(Some(Err(e))) => error!("{}", e),
                        }
                }
            }
        }
        connection.try_close().await;
    } else {
        warn!("failed to connect to provider");
    }

    debug!("asset worker has been dropped, stopping asset worker");
}

// If connect fails, the polling loop should be exited
async fn connect<C, P, E1, E2>(connector: &C, ids: &[String], opts: &PollOptions) -> Option<P>
where
    P: AssetInfoProvider<SubscriptionError = E1, PollingError = E2>,
    C: AssetInfoProviderConnector<Provider = P>,
{
    let mut retry_count: u64 = 1;

    let histogram = global::meter(opts.meter_name)
        .u64_histogram("connection-attempts")
        .with_unit("attempt")
        .build();

    while retry_count <= opts.max_retry {
        warn!("connect attempt {}", retry_count);

        if let Ok(mut provider) = connector.connect().await {
            if provider.subscribe(ids).await.is_ok() {
                histogram.record(retry_count, &[KeyValue::new("success", true)]);
                return Some(provider);
            }
        }

        histogram.record(retry_count, &[KeyValue::new("success", true)]);
        error!("failed to reconnect");

        retry_count += 1;
        sleep(opts.reconnect_buffer).await;
    }

    None
}
