use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

pub mod identity;
pub mod median;

#[derive(Debug, thiserror::Error)]
pub enum ProcessorError {
    #[error("invalid parameter: {0}")]
    InvalidParameterValue(String),

    #[error("invalid prerequisites amount")]
    InvalidPrerequisitesAmount,

    #[error("invalid price")]
    InvalidPrice(#[from] std::num::ParseFloatError),

    #[error("not enough sources")]
    NotEnoughSources,
}

/// The Processor trait defines the methods that a processor must implement.
#[enum_dispatch]
pub trait Processor {
    fn process(&self, data: Vec<f64>, prerequisites: Vec<f64>) -> Result<f64, ProcessorError>;
}

/// The Process enum represents the different types of processors that can be used.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
#[enum_dispatch(Processor)]
pub enum Process {
    Median(median::MedianProcessor),
    Identity(identity::IdentityProcessor),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_median() {
        let median = Process::Median(median::MedianProcessor {
            min_source_count: 1,
        });

        let res = median.process(vec![10.0, 20.0, 30.0, 40.0, 50.0], vec![]);

        assert_eq!(res.unwrap(), 30.0);
    }

    #[test]
    fn test_process_median_with_invalid_parameter() {
        let median = Process::Median(median::MedianProcessor {
            min_source_count: 0,
        });

        let res = median.process(vec![10.0, 20.0, 30.0, 40.0, 50.0], vec![]);

        assert!(res.is_err());
    }
}
