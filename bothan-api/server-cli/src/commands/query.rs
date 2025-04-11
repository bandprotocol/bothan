use std::collections::{HashMap, HashSet};
use std::error::Error as StdError;
use std::time::Duration;

use anyhow::{Context, anyhow};
use bothan_api::config::AppConfig;
use bothan_lib::types::AssetInfo;
use bothan_lib::worker::rest::AssetInfoProvider as RestAssetInfoProvider;
use bothan_lib::worker::websocket::{
    AssetInfoProvider as WebSocketAssetInfoProvider, AssetInfoProviderConnector, Data,
};
use clap::{Args, Parser, Subcommand};
use futures::stream::{FuturesOrdered, StreamExt};
use humantime::Duration as HumanDuration;
use itertools::Itertools;
use prettytable::{Table, row};
use tokio::time::timeout;

const DEFAULT_TIMEOUT: &str = "10s";
const CONFIG_ERROR_MESSAGE: &str = "config not found in config.toml, please refer to the example at /bothan-api/server-cli/config.example.toml";

#[derive(Parser)]
pub struct QueryCli {
    #[command(subcommand)]
    subcommand: QuerySubCommand,
}

#[derive(Args, Debug, Clone)]
pub struct QueryArgs {
    /// The list of query ids to query prices for
    pub query_ids: Vec<String>,

    /// Timeout duration
    #[arg(short, long, default_value = DEFAULT_TIMEOUT)]
    pub timeout: HumanDuration,
}

#[derive(Subcommand, Debug)]
pub enum QuerySubCommand {
    /// Query Binance prices
    Binance {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query Bitfinex prices
    Bitfinex {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query Bybit prices
    Bybit {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query Coinbase prices
    Coinbase {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query Coingecko prices
    Coingecko {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query Coinmarketcap prices
    Coinmarketcap {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query HTX prices
    Htx {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query Kraken prices
    Kraken {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query OKX prices
    Okx {
        #[clap(flatten)]
        args: QueryArgs,
    },
}

impl QueryCli {
    pub async fn run(&self, app_config: AppConfig) -> anyhow::Result<()> {
        match &self.subcommand {
            QuerySubCommand::Binance { args } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .binance
                    .with_context(|| format!("Binance {}", CONFIG_ERROR_MESSAGE))?;
                query_binance(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Bitfinex { args } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .bitfinex
                    .with_context(|| format!("Bitfinex {}", CONFIG_ERROR_MESSAGE))?;
                query_bitfinex(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Bybit { args } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .bybit
                    .with_context(|| format!("Bybit {}", CONFIG_ERROR_MESSAGE))?;
                query_bybit(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Coinbase { args } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .coinbase
                    .with_context(|| format!("Coinbase {}", CONFIG_ERROR_MESSAGE))?;
                query_coinbase(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Coingecko { args } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .coingecko
                    .with_context(|| format!("Coingecko {}", CONFIG_ERROR_MESSAGE))?;
                query_coingecko(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Coinmarketcap { args } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .coinmarketcap
                    .with_context(|| format!("Coinmarketcap {}", CONFIG_ERROR_MESSAGE))?;
                query_coinmarketcap(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Htx { args } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .htx
                    .with_context(|| format!("Htx {}", CONFIG_ERROR_MESSAGE))?;
                query_htx(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Kraken { args } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .kraken
                    .with_context(|| format!("Kraken {}", CONFIG_ERROR_MESSAGE))?;
                query_kraken(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Okx { args } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .okx
                    .with_context(|| format!("Okx {}", CONFIG_ERROR_MESSAGE))?;
                query_okx(opts, &args.query_ids, args.timeout).await?;
            }
        }

        Ok(())
    }
}

async fn query_binance<T: Into<Duration> + Clone>(
    opts: bothan_binance::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = bothan_binance::WebSocketConnector::new(opts.url);
    let asset_infos = query_websocket_in_chunks(
        connector,
        query_ids,
        opts.max_subscription_per_connection,
        timeout.into(),
    )
    .await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_bitfinex<T: Into<Duration>>(
    opts: bothan_bitfinex::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let api = bothan_bitfinex::api::builder::RestApiBuilder::new(opts.url).build()?;
    let asset_infos = query_rest_api(&api, query_ids, timeout.into()).await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_bybit<T: Into<Duration>>(
    opts: bothan_bybit::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = bothan_bybit::api::WebSocketConnector::new(opts.url);
    let asset_infos = query_single_websocket(connector, query_ids, timeout.into()).await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_coinbase<T: Into<Duration> + Clone>(
    opts: bothan_coinbase::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = bothan_coinbase::WebSocketConnector::new(opts.url);
    let asset_infos = query_websocket_in_chunks(
        connector,
        query_ids,
        opts.max_subscription_per_connection,
        timeout.into(),
    )
    .await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_coingecko<T: Into<Duration>>(
    opts: bothan_coingecko::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let api = bothan_coingecko::api::RestApiBuilder::new(opts.url, opts.user_agent, opts.api_key)
        .build()?;
    let asset_infos = query_rest_api(&api, query_ids, timeout.into()).await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_coinmarketcap<T: Into<Duration>>(
    opts: bothan_coinmarketcap::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let api = bothan_coinmarketcap::api::RestApiBuilder::new(opts.url, opts.api_key).build()?;
    let asset_infos = query_rest_api(&api, query_ids, timeout.into()).await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_htx<T: Into<Duration>>(
    opts: bothan_htx::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = bothan_htx::api::WebSocketConnector::new(opts.url);
    let asset_infos = query_single_websocket(connector, query_ids, timeout.into()).await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_kraken<T: Into<Duration>>(
    opts: bothan_kraken::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = bothan_kraken::api::WebSocketConnector::new(opts.url);
    let asset_infos = query_single_websocket(connector, query_ids, timeout.into()).await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_okx<T: Into<Duration>>(
    opts: bothan_okx::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = bothan_okx::api::WebSocketConnector::new(opts.url);
    let asset_infos = query_single_websocket(connector, query_ids, timeout.into()).await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_rest_api<P: RestAssetInfoProvider>(
    provider: &P,
    query_ids: &[String],
    timeout_interval: Duration,
) -> anyhow::Result<Vec<AssetInfo>>
where
    P::Error: StdError + Send + Sync + 'static,
{
    let deduped_ids = dedup_ids(query_ids);
    Ok(timeout(timeout_interval, provider.get_asset_info(&deduped_ids)).await??)
}

async fn query_single_websocket<P, E1, E2, C>(
    connector: C,
    ids: &[String],
    timeout: Duration,
) -> anyhow::Result<Vec<AssetInfo>>
where
    E1: StdError + Send + Sync + 'static,
    E2: StdError + Send + Sync + 'static,
    P: WebSocketAssetInfoProvider<SubscriptionError = E1, PollingError = E2>,
    C: AssetInfoProviderConnector<Provider = P, Error = E1>,
{
    let mut provider = connector.connect().await?;
    let deduped_ids = dedup_ids(ids);

    provider.subscribe(&deduped_ids).await?;
    query_websocket(&mut provider, deduped_ids, timeout).await
}

async fn query_websocket_in_chunks<C, P, E1, E2>(
    connector: C,
    ids: &[String],
    max_subscription_per_connection: usize,
    timeout: Duration,
) -> anyhow::Result<Vec<AssetInfo>>
where
    E1: StdError + Send + Sync + 'static,
    E2: StdError + Send + Sync + 'static,
    P: WebSocketAssetInfoProvider<SubscriptionError = E1, PollingError = E2>,
    C: AssetInfoProviderConnector<Provider = P, Error = E1>,
{
    let mut tasks = FuturesOrdered::new();
    let deduped_ids = dedup_ids(ids);

    for chunk in deduped_ids
        .into_iter()
        .chunks(max_subscription_per_connection)
        .into_iter()
    {
        let chunk_ids = chunk.collect::<Vec<String>>();
        let mut provider = connector.connect().await?;
        WebSocketAssetInfoProvider::subscribe(&mut provider, &chunk_ids).await?;

        tasks.push_back(async move { query_websocket(&mut provider, chunk_ids, timeout).await });
    }

    let results = tasks
        .collect::<Vec<Result<Vec<AssetInfo>, anyhow::Error>>>()
        .await;
    let asset_infos = results
        .into_iter()
        .collect::<Result<Vec<Vec<AssetInfo>>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<AssetInfo>>();

    Ok(asset_infos)
}

async fn query_websocket<E1, E2, P>(
    provider: &mut P,
    ids: Vec<String>,
    timeout_interval: Duration,
) -> anyhow::Result<Vec<AssetInfo>>
where
    E1: StdError + Send + Sync + 'static,
    E2: StdError + Send + Sync + 'static,
    P: WebSocketAssetInfoProvider<SubscriptionError = E1, PollingError = E2>,
{
    let mut asset_infos: HashMap<String, AssetInfo> = HashMap::with_capacity(ids.len());

    timeout(timeout_interval, async {
        while asset_infos.len() < ids.len() {
            if let Some(data) = provider.next().await {
                if let Data::AssetInfo(infos) = data? {
                    for info in infos {
                        asset_infos.insert(info.id.clone(), info);
                    }
                }
            } else {
                return Err(anyhow!("stream closed unexpectedly"));
            }
        }

        Ok(())
    })
    .await??;

    ids.iter()
        .map(|id| {
            asset_infos
                .remove(id)
                .ok_or_else(|| anyhow!("missing asset info for id: {}", id))
        })
        .collect()
}

fn dedup_ids(ids: &[String]) -> Vec<String> {
    let mut seen = HashSet::with_capacity(ids.len());
    let mut result = Vec::with_capacity(ids.len());

    for id in ids {
         if seen.insert(id) {
             result.push(id.clone());
         }
    } 

    result
}

fn display_asset_infos(asset_infos: Vec<AssetInfo>) {
    let mut table = Table::new();

    table.add_row(row!["ID", "Price", "Timestamp"]);

    for asset in asset_infos {
        table.add_row(row![
            asset.id,
            asset.price.to_string(),
            asset.timestamp.to_string()
        ]);
    }

    table.printstd();
}
