use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

pub mod identity;
pub mod median;

#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("invalid prerequisites amount")]
    InvalidPrerequisitesAmount,

    #[error("invalid price")]
    InvalidPrice(#[from] std::num::ParseFloatError),

    #[error("not enough sources")]
    NotEnoughSources,
}

#[enum_dispatch]
pub trait Processing {
    fn process(&self, data: Vec<f64>, prerequisites: Vec<f64>) -> Result<f64, ProcessingError>;
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
#[enum_dispatch(Processing)]
pub enum Processor {
    Median(median::MedianProcessor),
    Identity(identity::IdentityProcessor),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let median = Processor::Median(median::MedianProcessor {
            min_source_count: 0,
        });

        let res = median.process(vec![10.0, 20.0, 30.0, 40.0, 50.0], vec![]);

        assert_eq!(res.unwrap(), 30.0);
    }
}
