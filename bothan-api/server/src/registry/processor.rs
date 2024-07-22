use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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
pub trait Processor<T> {
    fn process(&self, data: Vec<T>) -> Result<T, ProcessorError>;
}

/// The Process enum represents the different types of processors that can be used.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
// #[enum_dispatch(Processor)]
pub enum Process {
    Median(median::MedianProcessor),
}

impl Processor<Decimal> for Process {
    fn process(&self, data: Vec<Decimal>) -> Result<Decimal, ProcessorError> {
        match self {
            Process::Median(median) => median.process(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Vec<Decimal> {
        vec![
            Decimal::from(10),
            Decimal::from(20),
            Decimal::from(30),
            Decimal::from(40),
            Decimal::from(50),
        ]
    }

    #[test]
    fn test_process_median() {
        let median = Process::Median(median::MedianProcessor {
            min_source_count: 1,
        });
        let res = median.process(data());

        assert_eq!(res.unwrap(), Decimal::from(30));
    }

    #[test]
    fn test_process_median_with_invalid_parameter() {
        let median = Process::Median(median::MedianProcessor {
            min_source_count: 0,
        });
        let res = median.process(data());

        assert!(res.is_err());
    }
}
