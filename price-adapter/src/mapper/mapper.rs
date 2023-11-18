use serde_json::Value;
use std::collections::HashMap;

pub trait Mapper {
    fn get_mapping(&self) -> &HashMap<String, Value>;
}
