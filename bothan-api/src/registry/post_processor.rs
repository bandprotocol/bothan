use std::collections::HashMap;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::post_processors::tick::TickPostProcessor;
#[allow(unused_imports)]
use crate::post_processors::PostProcessor as Trait;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
#[enum_dispatch(Trait)]
pub enum Function {
    TickConvertor(TickPostProcessor),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PostProcessor {
    pub function: Function,
    pub params: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json_str = r#"{ "function": "tick_convertor", "params": { } }"#;
        let expected_function = Function::TickConvertor(TickPostProcessor {});

        let deserialized = serde_json::from_str::<Function>(json_str);
        assert_eq!(deserialized.unwrap(), expected_function);
    }

    #[test]
    fn test_deserialize_with_invalid_parameter() {
        let json_str = r#"{ "function": "median", "params": { "test": "Jesus" } }"#;

        let deserialized = serde_json::from_str::<crate::registry::processor::Function>(json_str);
        assert!(deserialized.is_err());
    }
}
