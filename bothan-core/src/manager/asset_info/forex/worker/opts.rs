//! Worker options for configuring crypto asset source workers.

#[derive(Clone)]
pub enum ForexAssetWorkerOpts {
    Band(bothan_band::WorkerOpts),
}

impl ForexAssetWorkerOpts {
    pub fn name(&self) -> &str {
        match self {
            ForexAssetWorkerOpts::Band(opts) => opts.name(),
        }
    }
}

impl From<bothan_band::WorkerOpts> for ForexAssetWorkerOpts {
    fn from(value: bothan_band::WorkerOpts) -> Self {
        ForexAssetWorkerOpts::Band(value)
    }
}
