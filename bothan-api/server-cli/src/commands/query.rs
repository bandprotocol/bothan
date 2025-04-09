use std::collections::{HashMap, HashSet};
use std::error::Error as StdError;
use std::time::Duration;

use anyhow::anyhow;
use bothan_api::config::AppConfig;
use bothan_lib::types::AssetInfo;
use bothan_lib::worker::rest::AssetInfoProvider as RestAssetInfoProvider;
use bothan_lib::worker::websocket::{AssetInfoProvider as WebSocketAssetInfoProvider, Data};
use clap::{Parser, Subcommand};
use futures::stream::{FuturesOrdered, StreamExt};
use humantime::Duration as HumanDuration;
use itertools::Itertools;
use tokio::time::timeout;

#[derive(Parser)]
pub struct QueryCli {
    #[command(subcommand)]
    subcommand: QuerySubCommand,
}

#[derive(Subcommand)]
enum QuerySubCommand {
    /// Query Binance prices
    Binance {
        /// The list of query ids to query prices for
        query_ids: Vec<String>,

        /// Timeout duration
        #[arg(short, long, default_value = "60s")]
        timeout: HumanDuration,
    },
    /// Query Bitfinex prices
    Bitfinex {
        /// The list of query ids to query prices for
        query_ids: Vec<String>,

        /// Timeout duration
        #[arg(short, long, default_value = "60s")]
        timeout: HumanDuration,
    },

    /// Query Bybit prices
    Bybit {
        /// The list of query ids to query prices for
        query_ids: Vec<String>,

        /// Timeout duration
        #[arg(short, long, default_value = "60s")]
        timeout: HumanDuration,
    },

    /// Query Coinbase prices
    Coinbase {
        /// The list of query ids to query prices for
        query_ids: Vec<String>,

        /// Timeout duration
        #[arg(short, long, default_value = "60s")]
        timeout: HumanDuration,
    },

    /// Query Coingecko prices
    Coingecko {
        /// The list of query ids to query prices for
        query_ids: Vec<String>,

        /// Timeout duration
        #[arg(short, long, default_value = "60s")]
        timeout: HumanDuration,
    },

    /// Query Coinmarketcap prices
    Coinmarketcap {
        /// The list of query ids to query prices for
        query_ids: Vec<String>,

        /// Timeout duration
        #[arg(short, long, default_value = "60s")]
        timeout: HumanDuration,
    },

    /// Query HTX prices
    Htx {
        /// The list of query ids to query prices for
        query_ids: Vec<String>,

        /// Timeout duration
        #[arg(short, long, default_value = "60s")]
        timeout: HumanDuration,
    },

    /// Query Kraken prices
    Kraken {
        /// The list of query ids to query prices for
        query_ids: Vec<String>,

        /// Timeout duration
        #[arg(short, long, default_value = "60s")]
        timeout: HumanDuration,
    },

    /// Query OKX prices
    Okx {
        /// The list of query ids to query prices for
        query_ids: Vec<String>,

        /// Timeout duration
        #[arg(short, long, default_value = "60s")]
        timeout: HumanDuration,
    },
}

impl QueryCli {
    pub async fn run(&self, app_config: AppConfig) -> anyhow::Result<()> {
        match &self.subcommand {
            QuerySubCommand::Binance { query_ids, timeout } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .binance
                    .ok_or_else(|| anyhow!("Binance config not found"))?;

                query_binance(opts, query_ids, *timeout).await?;
            }
            QuerySubCommand::Bitfinex { query_ids, timeout } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .bitfinex
                    .ok_or_else(|| anyhow!("Bitfinex config not found"))?;
                query_bitfinex(opts, query_ids, *timeout).await?
            }
            QuerySubCommand::Bybit { query_ids, timeout } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .bybit
                    .ok_or_else(|| anyhow!("Bybit config not found"))?;
                query_bybit(opts, query_ids, *timeout).await?
            }
            QuerySubCommand::Coinbase { query_ids, timeout } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .coinbase
                    .ok_or_else(|| anyhow!("Coinbase config not found"))?;
                query_coinbase(opts, query_ids, *timeout).await?
            }
            QuerySubCommand::Coingecko { query_ids, timeout } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .coingecko
                    .ok_or_else(|| anyhow!("Coingecko config not found"))?;
                query_coingecko(opts, query_ids, *timeout).await?
            }
            QuerySubCommand::Coinmarketcap { query_ids, timeout } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .coinmarketcap
                    .ok_or_else(|| anyhow!("Coinmarketcap config not found"))?;
                query_coinmarketcap(opts, query_ids, *timeout).await?
            }
            QuerySubCommand::Htx { query_ids, timeout } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .htx
                    .ok_or_else(|| anyhow!("Htx config not found"))?;
                query_htx(opts, query_ids, *timeout).await?
            }
            QuerySubCommand::Kraken { query_ids, timeout } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .kraken
                    .ok_or_else(|| anyhow!("Kraken config not found"))?;
                query_kraken(opts, query_ids, *timeout).await?
            }
            QuerySubCommand::Okx { query_ids, timeout } => {
                let opts = app_config
                    .manager
                    .crypto
                    .source
                    .okx
                    .ok_or_else(|| anyhow!("Okx config not found"))?;
                query_okx(opts, query_ids, *timeout).await?
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

    let mut tasks = FuturesOrdered::new();

    let distinct_ids = get_distinct_ids(query_ids);

    for chunk in distinct_ids
        .into_iter()
        .chunks(opts.max_subscription_per_connection)
        .into_iter()
    {
        let cloned_timeout = timeout.clone().into();
        let chunk_ids = chunk.collect::<Vec<String>>();
        let mut provider = connector.connect().await?;
        WebSocketAssetInfoProvider::subscribe(&mut provider, &chunk_ids).await?;

        tasks.push_back(async move { query(&mut provider, chunk_ids, cloned_timeout).await });
    }

    let results = tasks
        .collect::<Vec<Result<Vec<AssetInfo>, anyhow::Error>>>()
        .await;

    let asset_infos = results
        .into_iter()
        .collect::<Result<Vec<Vec<AssetInfo>>, anyhow::Error>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<AssetInfo>>();

    print_asset_infos(asset_infos);

    Ok(())
}

async fn query_bitfinex<T: Into<Duration>>(
    opts: bothan_bitfinex::WorkerOpts,
    query_ids: &[String],
    timeout_interval: T,
) -> anyhow::Result<()> {
    let api = bothan_bitfinex::api::builder::RestApiBuilder::new(opts.url).build()?;

    let distinct_ids = get_distinct_ids(query_ids);
    let asset_infos = timeout(
        timeout_interval.into(),
        RestAssetInfoProvider::get_asset_info(&api, &distinct_ids),
    )
    .await??;

    print_asset_infos(asset_infos);
    Ok(())
}

async fn query_bybit<T: Into<Duration>>(
    opts: bothan_bybit::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = bothan_bybit::WebSocketConnector::new(opts.url);
    let mut provider = connector.connect().await?;

    let distinct_ids = get_distinct_ids(query_ids);
    WebSocketAssetInfoProvider::subscribe(&mut provider, &distinct_ids).await?;
    let asset_infos = query(&mut provider, distinct_ids, timeout.into()).await?;

    print_asset_infos(asset_infos);
    Ok(())
}

async fn query_coinbase<T: Into<Duration> + Clone>(
    opts: bothan_coinbase::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = bothan_coinbase::WebSocketConnector::new(opts.url);

    let mut tasks = FuturesOrdered::new();

    let distinct_ids = get_distinct_ids(query_ids);

    for chunk in distinct_ids
        .into_iter()
        .chunks(opts.max_subscription_per_connection)
        .into_iter()
    {
        let cloned_timeout = timeout.clone().into();
        let chunk_ids = chunk.collect::<Vec<String>>();
        let mut provider = connector.connect().await?;
        WebSocketAssetInfoProvider::subscribe(&mut provider, &chunk_ids).await?;

        tasks.push_back(async move { query(&mut provider, chunk_ids, cloned_timeout).await });
    }

    let results = tasks
        .collect::<Vec<Result<Vec<AssetInfo>, anyhow::Error>>>()
        .await;

    let asset_infos = results
        .into_iter()
        .collect::<Result<Vec<Vec<AssetInfo>>, anyhow::Error>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<AssetInfo>>();

    print_asset_infos(asset_infos);

    Ok(())
}

async fn query_coingecko<T: Into<Duration>>(
    opts: bothan_coingecko::WorkerOpts,
    query_ids: &[String],
    timeout_interval: T,
) -> anyhow::Result<()> {
    let api = bothan_coingecko::api::RestApiBuilder::new(opts.url, opts.user_agent, opts.api_key)
        .build()?;

    let distinct_ids = get_distinct_ids(query_ids);
    let asset_infos = timeout(
        timeout_interval.into(),
        RestAssetInfoProvider::get_asset_info(&api, &distinct_ids),
    )
    .await??;

    print_asset_infos(asset_infos);
    Ok(())
}

async fn query_coinmarketcap<T: Into<Duration>>(
    opts: bothan_coinmarketcap::WorkerOpts,
    query_ids: &[String],
    timeout_interval: T,
) -> anyhow::Result<()> {
    let api = bothan_coinmarketcap::api::RestApiBuilder::new(opts.url, opts.api_key).build()?;

    let distinct_ids = get_distinct_ids(query_ids);
    let asset_infos = timeout(
        timeout_interval.into(),
        RestAssetInfoProvider::get_asset_info(&api, &distinct_ids),
    )
    .await??;

    print_asset_infos(asset_infos);
    Ok(())
}

async fn query_htx<T: Into<Duration>>(
    opts: bothan_htx::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = bothan_htx::api::WebSocketConnector::new(opts.url);
    let mut provider = connector.connect().await?;

    let distinct_ids = get_distinct_ids(query_ids);
    WebSocketAssetInfoProvider::subscribe(&mut provider, &distinct_ids).await?;
    let asset_infos = query(&mut provider, distinct_ids, timeout.into()).await?;

    print_asset_infos(asset_infos);
    Ok(())
}

async fn query_kraken<T: Into<Duration>>(
    opts: bothan_kraken::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = bothan_kraken::api::WebSocketConnector::new(opts.url);
    let mut provider = connector.connect().await?;

    let distinct_ids = get_distinct_ids(query_ids);
    WebSocketAssetInfoProvider::subscribe(&mut provider, &distinct_ids).await?;
    let asset_infos = query(&mut provider, distinct_ids, timeout.into()).await?;

    print_asset_infos(asset_infos);
    Ok(())
}

async fn query_okx<T: Into<Duration>>(
    opts: bothan_okx::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = bothan_okx::api::WebSocketConnector::new(opts.url);
    let mut provider = connector.connect().await?;

    let distinct_ids = get_distinct_ids(query_ids);
    WebSocketAssetInfoProvider::subscribe(&mut provider, &distinct_ids).await?;
    let asset_infos = query(&mut provider, distinct_ids, timeout.into()).await?;

    print_asset_infos(asset_infos);
    Ok(())
}

fn get_distinct_ids(ids: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut result = Vec::with_capacity(ids.len());

    for id in ids {
        if seen.insert(id) {
            result.push(id.clone());
        }
    }

    result
}

async fn query<E1, E2, P>(
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
        println!("ids.len(): {}", ids.len());
        println!("asset_infos.len(): {}", asset_infos.len());
        while asset_infos.len() < ids.len() {
            match provider.next().await {
                Some(Ok(Data::AssetInfo(infos))) => {
                    println!("hello");
                    for info in infos {
                        asset_infos.insert(info.id.clone(), info);
                    }
                }
                Some(Ok(Data::Ping | Data::Unused)) => {
                    println!("ping");
                }
                Some(Err(e)) => return Err(anyhow!(e)),
                None => return Err(anyhow!("stream closed unexpectedly")),
            }
        }

        ids.iter()
            .map(|id| {
                asset_infos
                    .get(id)
                    .cloned()
                    .ok_or_else(|| anyhow!("missing asset info for id: {}", id))
            })
            .collect::<Result<Vec<AssetInfo>, anyhow::Error>>()
    })
    .await?
}

fn print_asset_infos(asset_infos: Vec<AssetInfo>) {
    for asset in asset_infos {
        println!("- id: {}", asset.id);
        println!("  price: {}", asset.price);
        println!("  timestamp: {}", asset.timestamp);
    }
}
