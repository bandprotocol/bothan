use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod tick;

#[derive(Debug, Error, PartialEq, Clone)]
#[error("{msg}")]
pub struct PostProcessError {
    msg: String,
}

impl PostProcessError {
    pub fn new<T: Into<String>>(msg: T) -> Self {
        PostProcessError { msg: msg.into() }
    }
}

/// The PostProcessor trait defines the methods that a post-processor must implement.
pub trait PostProcess<T> {
    fn process(&self, data: T) -> Result<T, PostProcessError>;
}

/// The PostProcess enum represents the different types of post-processors that can be used.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
pub enum PostProcessor {
    TickConvertor(tick::TickPostProcessor),
}

impl PostProcess<Decimal> for PostProcessor {
    fn process(&self, data: Decimal) -> Result<Decimal, PostProcessError> {
        match self {
            PostProcessor::TickConvertor(tick) => tick.process(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use tick::TickPostProcessor;

    use super::*;

    #[test]
    fn test_process() {
        let tick_convertor = PostProcessor::TickConvertor(TickPostProcessor {});
        let result = tick_convertor.process(Decimal::from(10));
        assert_eq!(
            result.unwrap(),
            Decimal::from_str_exact("285171.00220329970405837917374").unwrap()
        );
    }

    #[test]
    fn test_deserialize() {
        let json_str = r#"{ "function": "tick_convertor", "params": { } }"#;
        let expected_function = PostProcessor::TickConvertor(TickPostProcessor {});

        let deserialized = serde_json::from_str::<PostProcessor>(json_str);
        assert_eq!(deserialized.unwrap(), expected_function);
    }

    #[test]
    fn test_deserialize_with_invalid_parameter() {
        let json_str = r#"{ "function": "median", "params": { "test": "Jesus" } }"#;

        let deserialized = serde_json::from_str::<PostProcessor>(json_str);
        assert!(deserialized.is_err());
    }
}
