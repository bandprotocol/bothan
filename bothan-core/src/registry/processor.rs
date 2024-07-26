use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod median;

#[derive(Debug, Error, PartialEq, Clone)]
#[error("{msg}")]
pub struct ProcessError {
    msg: String,
}

impl ProcessError {
    pub fn new<T: Into<String>>(msg: T) -> Self {
        ProcessError { msg: msg.into() }
    }
}

/// The Processor trait defines the methods that a processor must implement.
pub trait Process<T> {
    fn process(&self, data: Vec<T>) -> Result<T, ProcessError>;
}

/// The Process enum represents the different types of processors that can be used.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
// #[enum_dispatch(Processor)]
pub enum Processor {
    Median(median::MedianProcessor),
}

impl Process<Decimal> for Processor {
    fn process(&self, data: Vec<Decimal>) -> Result<Decimal, ProcessError> {
        match self {
            Processor::Median(median) => median.process(data),
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
        let median = Processor::Median(median::MedianProcessor {
            min_source_count: 1,
        });
        let res = median.process(data());

        assert_eq!(res.unwrap(), Decimal::from(30));
    }

    #[test]
    fn test_process_median_with_invalid_parameter() {
        let median = Processor::Median(median::MedianProcessor {
            min_source_count: 0,
        });
        let res = median.process(data());

        assert!(res.is_err());
    }
}
