use crate::error::Error;
use serde_json::Value;
use std::collections::HashMap;

pub trait Mapper {
    fn get_mapping(&self) -> Result<&HashMap<String, Value>, Error>;
}
