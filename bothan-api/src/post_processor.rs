use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

pub mod tick;

#[derive(Debug, thiserror::Error)]
pub enum PostProcessingError {
    #[error("price out of bound")]
    OutOfBound,
}

#[enum_dispatch]
pub trait PostProcessing {
    fn process(&self, data: f64) -> Result<f64, PostProcessingError>;
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
#[enum_dispatch(PostProcessing)]
pub enum PostProcessor {
    TickConvertor(tick::TickPostProcessor),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::post_processor::tick::TickPostProcessor;

    #[test]
    fn test_process() {
        let tick_convertor = PostProcessor::TickConvertor(TickPostProcessor {});
        let result = tick_convertor.process(10.0);
        assert_eq!(result.unwrap(), 285171.0022033022);
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
