use std::collections::HashMap;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::processors::identity::IdentityProcessor;
use crate::processors::median::MedianProcessor;
#[allow(unused_imports)]
use crate::processors::Processor as Trait;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
#[enum_dispatch(Trait)]
pub enum Function {
    Median(MedianProcessor),
    Identity(IdentityProcessor),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Processor {
    pub function: Function,
    pub params: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json_str = r#"{ "function": "median", "params": { "min_source_count": 3 } }"#;
        let expected_function = Function::Median(MedianProcessor {
            min_source_count: 3,
        });

        let deserialized = serde_json::from_str::<Function>(json_str);
        assert_eq!(deserialized.unwrap(), expected_function);
    }

    #[test]
    fn test_deserialize_with_missing_parameter() {
        let json_str = r#"{ "function": "median", "params": { } }"#;

        let deserialized = serde_json::from_str::<Function>(json_str);
        assert!(deserialized.is_err());
    }
}
