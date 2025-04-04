use bothan_api::config::AppConfig;
use bothan_lib::worker::{rest, websocket};
use clap::{Parser, Subcommand};
use humantime::Duration as HumanDuration;
use itertools::Itertools;

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
                query_binance(&app_config, query_ids, timeout).await?
            }
            QuerySubCommand::Bitfinex { query_ids, timeout } => {
                query_bitfinex(&app_config, query_ids, timeout).await?
            }
            QuerySubCommand::Bybit { query_ids, timeout } => {
                query_bybit(&app_config, query_ids, timeout).await?
            }
            QuerySubCommand::Coinbase { query_ids, timeout } => {
                query_coinbase(&app_config, query_ids, timeout).await?
            }
            QuerySubCommand::Coingecko { query_ids, timeout } => {
                query_coingecko(&app_config, query_ids, timeout).await?
            }
            QuerySubCommand::Coinmarketcap { query_ids, timeout } => {
                query_coinmarketcap(&app_config, query_ids, timeout).await?
            }
            QuerySubCommand::Htx { query_ids, timeout } => {
                query_htx(&app_config, query_ids, timeout).await?
            }
            QuerySubCommand::Kraken { query_ids, timeout } => {
                query_kraken(&app_config, query_ids, timeout).await?
            }
            QuerySubCommand::Okx { query_ids, timeout } => {
                query_okx(&app_config, query_ids, timeout).await?
            }
        }

        Ok(())
    }
}

async fn query_binance(
    config: &AppConfig,
    query_ids: &[String],
    timeout: &humantime::Duration,
) -> anyhow::Result<()> {
    let opts = config.manager.crypto.source.binance.clone().unwrap();
    let connector = bothan_binance::WebSocketConnector::new(opts.url);
    let mut all_infos = vec![];

    for chunk in &query_ids
        .iter()
        .cloned()
        .chunks(opts.max_subscription_per_connection)
    {
        let infos = websocket::query(&connector, chunk.collect(), (*timeout).into()).await?;
        all_infos.extend(infos);
    }

    println!("{:#?}", all_infos);
    Ok(())
}

async fn query_bitfinex(
    config: &AppConfig,
    query_ids: &[String],
    timeout: &humantime::Duration,
) -> anyhow::Result<()> {
    let opts = config.manager.crypto.source.bitfinex.clone().unwrap();
    let api = bothan_bitfinex::api::builder::RestApiBuilder::new(opts.url).build()?;
    let asset_info = rest::query(&api, query_ids, (*timeout).into()).await?;
    println!("{:#?}", asset_info);
    Ok(())
}

async fn query_bybit(
    config: &AppConfig,
    query_ids: &[String],
    timeout: &humantime::Duration,
) -> anyhow::Result<()> {
    let opts = config.manager.crypto.source.bybit.clone().unwrap();
    let connector = bothan_bybit::WebSocketConnector::new(opts.url);
    let asset_info = websocket::query(&connector, query_ids.to_vec(), (*timeout).into()).await?;
    println!("{:#?}", asset_info);
    Ok(())
}

async fn query_coinbase(
    config: &AppConfig,
    query_ids: &[String],
    timeout: &humantime::Duration,
) -> anyhow::Result<()> {
    let opts = config.manager.crypto.source.coinbase.clone().unwrap();
    let connector = bothan_coinbase::WebSocketConnector::new(opts.url);
    let mut all_infos = vec![];
    for chunk in &query_ids
        .iter()
        .cloned()
        .chunks(opts.max_subscription_per_connection)
    {
        let infos = websocket::query(&connector, chunk.collect(), (*timeout).into()).await?;
        all_infos.extend(infos);
    }
    println!("{:#?}", all_infos);
    Ok(())
}

async fn query_coingecko(
    config: &AppConfig,
    query_ids: &[String],
    timeout: &humantime::Duration,
) -> anyhow::Result<()> {
    let opts = config.manager.crypto.source.coingecko.clone().unwrap();
    let api = bothan_coingecko::api::RestApiBuilder::new(opts.url, opts.user_agent, opts.api_key)
        .build()?;
    let asset_info = rest::query(&api, query_ids, (*timeout).into()).await?;
    println!("{:#?}", asset_info);
    Ok(())
}

async fn query_coinmarketcap(
    config: &AppConfig,
    query_ids: &[String],
    timeout: &humantime::Duration,
) -> anyhow::Result<()> {
    let opts = config.manager.crypto.source.coinmarketcap.clone().unwrap();
    let api = bothan_coinmarketcap::api::RestApiBuilder::new(opts.url, opts.api_key).build()?;
    let asset_info = rest::query(&api, query_ids, (*timeout).into()).await?;
    println!("{:#?}", asset_info);
    Ok(())
}

async fn query_htx(
    config: &AppConfig,
    query_ids: &[String],
    timeout: &humantime::Duration,
) -> anyhow::Result<()> {
    let opts = config.manager.crypto.source.htx.clone().unwrap();
    let connector = bothan_htx::api::WebSocketConnector::new(opts.url);
    let asset_info = websocket::query(&connector, query_ids.to_vec(), (*timeout).into()).await?;
    println!("{:#?}", asset_info);
    Ok(())
}

async fn query_kraken(
    config: &AppConfig,
    query_ids: &[String],
    timeout: &humantime::Duration,
) -> anyhow::Result<()> {
    let opts = config.manager.crypto.source.kraken.clone().unwrap();
    let connector = bothan_kraken::WebSocketConnector::new(opts.url);
    let asset_info = websocket::query(&connector, query_ids.to_vec(), (*timeout).into()).await?;
    println!("{:#?}", asset_info);
    Ok(())
}

async fn query_okx(
    config: &AppConfig,
    query_ids: &[String],
    timeout: &humantime::Duration,
) -> anyhow::Result<()> {
    let opts = config.manager.crypto.source.okx.clone().unwrap();
    let connector = bothan_okx::WebSocketConnector::new(opts.url);
    let asset_info = websocket::query(&connector, query_ids.to_vec(), (*timeout).into()).await?;
    println!("{:#?}", asset_info);
    Ok(())
}
