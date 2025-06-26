//! Bothan CLI query subcommand module.
//!
//! Query prices and asset info from supported exchanges and data sources.
//!
//! Supports querying prices from multiple exchanges and data sources with customizable timeout and pretty-printed output.
//!
//! ## Features
//!
//! - Query prices from Binance, Bitfinex, Bybit, Coinbase, CoinGecko, CoinMarketCap, HTX, Kraken, OKX
//! - Customizable timeout and query IDs
//! - Pretty-printed table output
//!
//! ## Usage
//!
//! ```bash
//! bothan query binance BTCUSDT ETHUSDT
//! bothan query coingecko bitcoin ethereum
//! ```

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;
use bothan_api::config::AppConfig;
use bothan_lib::types::AssetInfo;
use bothan_lib::worker::rest::AssetInfoProvider as RestAssetInfoProvider;
use bothan_lib::worker::websocket::{
    AssetInfoProvider as WebSocketAssetInfoProvider, AssetInfoProviderConnector, Data,
};
use clap::{Args, Parser, Subcommand};
use futures::stream::{FuturesUnordered, TryStreamExt};
use humantime::Duration as HumanDuration;
use itertools::Itertools;
use prettytable::{Table, row};
use tokio::time::timeout;

const DEFAULT_TIMEOUT: &str = "10s";

#[derive(Parser)]
/// CLI arguments for the `query` command.
pub struct QueryCli {
    #[command(subcommand)]
    subcommand: QuerySubCommand,
}

#[derive(Args, Debug, Clone)]
/// Arguments for querying prices.
pub struct QueryArgs {
    /// The list of query ids to query prices for
    pub query_ids: Vec<String>,

    /// Timeout duration
    #[arg(short, long, default_value = DEFAULT_TIMEOUT)]
    pub timeout: HumanDuration,
}
#[derive(Subcommand, Debug)]
/// Supported query subcommands for each exchange or data source.
pub enum QuerySubCommand {
    /// Query Binance prices
    #[clap(name = "binance")]
    Binance {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query Bitfinex prices
    #[clap(name = "bitfinex")]
    Bitfinex {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query Bybit prices
    #[clap(name = "bybit")]
    Bybit {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query Coinbase prices
    #[clap(name = "coinbase")]
    Coinbase {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query CoinGecko prices
    #[clap(name = "coingecko")]
    CoinGecko {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query CoinMarketCap prices
    #[clap(name = "coinmarketcap")]
    CoinMarketCap {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query HTX prices
    #[clap(name = "htx")]
    Htx {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query Kraken prices
    #[clap(name = "kraken")]
    Kraken {
        #[clap(flatten)]
        args: QueryArgs,
    },
    /// Query OKX prices
    #[clap(name = "okx")]
    Okx {
        #[clap(flatten)]
        args: QueryArgs,
    },
}

impl QueryCli {
    pub async fn run(&self, app_config: AppConfig) -> anyhow::Result<()> {
        let source_config = app_config.manager.crypto.source;
        let config_err = anyhow!("Config is missing. Please check your config.toml.");
        match &self.subcommand {
            QuerySubCommand::Binance { args } => {
                let opts = source_config.binance.ok_or(config_err)?;
                query_binance(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Bitfinex { args } => {
                let opts = source_config.bitfinex.ok_or(config_err)?;
                query_bitfinex(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Bybit { args } => {
                let opts = source_config.bybit.ok_or(config_err)?;
                query_bybit(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Coinbase { args } => {
                let opts = source_config.coinbase.ok_or(config_err)?;
                query_coinbase(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::CoinGecko { args } => {
                let opts = source_config.coingecko.ok_or(config_err)?;
                query_coingecko(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::CoinMarketCap { args } => {
                let opts = source_config.coinmarketcap.ok_or(config_err)?;
                query_coinmarketcap(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Htx { args } => {
                let opts = source_config.htx.ok_or(config_err)?;
                query_htx(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Kraken { args } => {
                let opts = source_config.kraken.ok_or(config_err)?;
                query_kraken(opts, &args.query_ids, args.timeout).await?;
            }
            QuerySubCommand::Okx { args } => {
                let opts = source_config.okx.ok_or(config_err)?;
                query_okx(opts, &args.query_ids, args.timeout).await?;
            }
        }

        Ok(())
    }
}

async fn query_binance<T: Into<Duration>>(
    opts: bothan_binance::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = Arc::new(bothan_binance::WebSocketConnector::new(opts.url));
    let asset_infos = query_websocket_with_max_sub(
        connector,
        dedup(query_ids),
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
    timeout_interval: T,
) -> anyhow::Result<()> {
    let api = bothan_bitfinex::api::builder::RestApiBuilder::new(opts.url).build()?;
    let asset_infos = timeout(
        timeout_interval.into(),
        api.get_asset_info(&dedup(query_ids)),
    )
    .await??;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_bybit<T: Into<Duration>>(
    opts: bothan_bybit::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = Arc::new(bothan_bybit::api::WebSocketConnector::new(opts.url));
    let asset_infos = query_websocket(connector, dedup(query_ids), timeout.into()).await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_coinbase<T: Into<Duration>>(
    opts: bothan_coinbase::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = Arc::new(bothan_coinbase::WebSocketConnector::new(opts.url));
    let asset_infos = query_websocket_with_max_sub(
        connector,
        dedup(query_ids),
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
    timeout_interval: T,
) -> anyhow::Result<()> {
    let api = bothan_coingecko::api::RestApiBuilder::new(opts.url, opts.user_agent, opts.api_key)
        .build()?;
    let asset_infos = timeout(
        timeout_interval.into(),
        api.get_asset_info(&dedup(query_ids)),
    )
    .await??;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_coinmarketcap<T: Into<Duration>>(
    opts: bothan_coinmarketcap::WorkerOpts,
    query_ids: &[String],
    timeout_interval: T,
) -> anyhow::Result<()> {
    let api = bothan_coinmarketcap::api::RestApiBuilder::new(opts.url, opts.api_key).build()?;
    let asset_infos = timeout(
        timeout_interval.into(),
        api.get_asset_info(&dedup(query_ids)),
    )
    .await??;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_htx<T: Into<Duration>>(
    opts: bothan_htx::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = Arc::new(bothan_htx::api::WebSocketConnector::new(opts.url));
    let asset_infos = query_websocket(connector, dedup(query_ids), timeout.into()).await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_kraken<T: Into<Duration>>(
    opts: bothan_kraken::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = Arc::new(bothan_kraken::api::WebSocketConnector::new(opts.url));
    let asset_infos = query_websocket(connector, dedup(query_ids), timeout.into()).await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_okx<T: Into<Duration>>(
    opts: bothan_okx::WorkerOpts,
    query_ids: &[String],
    timeout: T,
) -> anyhow::Result<()> {
    let connector = Arc::new(bothan_okx::api::WebSocketConnector::new(opts.url));
    let asset_infos = query_websocket(connector, dedup(query_ids), timeout.into()).await?;

    display_asset_infos(asset_infos);
    Ok(())
}

async fn query_websocket_with_max_sub<C, P, E1, E2>(
    connector: Arc<C>,
    ids: Vec<String>,
    max_subscription_per_connection: usize,
    timeout: Duration,
) -> anyhow::Result<Vec<AssetInfo>>
where
    E1: Error + Send + Sync + 'static,
    E2: Error + Send + Sync + 'static,
    P: WebSocketAssetInfoProvider<SubscriptionError = E1, ListeningError = E2>,
    C: AssetInfoProviderConnector<Provider = P, Error = E1>,
{
    let tasks = FuturesUnordered::new();

    for chunk in &ids.into_iter().chunks(max_subscription_per_connection) {
        let chunk_ids = chunk.collect();
        let cloned_connector = connector.clone();

        tasks.push(async move { query_websocket(cloned_connector, chunk_ids, timeout).await });
    }

    let asset_infos = tasks
        .try_collect::<Vec<Vec<AssetInfo>>>()
        .await?
        .into_iter()
        .flatten()
        .collect();

    Ok(asset_infos)
}

async fn query_websocket<E1, E2, P, C>(
    connector: Arc<C>,
    ids: Vec<String>,
    timeout_interval: Duration,
) -> anyhow::Result<Vec<AssetInfo>>
where
    E1: Error + Send + Sync + 'static,
    E2: Error + Send + Sync + 'static,
    P: WebSocketAssetInfoProvider<SubscriptionError = E1, ListeningError = E2>,
    C: AssetInfoProviderConnector<Provider = P, Error = E1>,
{
    let mut provider = connector.connect().await?;
    provider.subscribe(&ids).await?;

    let mut asset_infos: HashMap<String, AssetInfo> = HashMap::with_capacity(ids.len());

    timeout(timeout_interval, async {
        while asset_infos.len() < ids.len() {
            let data = provider.next().await?;

            if let Ok(Data::AssetInfo(infos)) = data {
                for info in infos {
                    asset_infos.insert(info.id.clone(), info);
                }
            }
        }
        Some(())
    })
    .await?
    .ok_or(anyhow!("stream closed unexpectedly"))?;

    Ok(asset_infos.into_values().collect())
}

fn dedup(ids: &[String]) -> Vec<String> {
    let mut seen = HashSet::with_capacity(ids.len());
    let mut dedup = Vec::with_capacity(ids.len());

    for id in ids {
        if seen.insert(id) {
            dedup.push(id.clone());
        }
    }

    dedup
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
