use std::collections::HashMap;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::post_processors::tick::TickPostProcessor;
#[allow(unused_imports)]
use crate::post_processors::PostProcessor as Trait;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[enum_dispatch(Trait)]
pub enum Function {
    TickConvertor(TickPostProcessor),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PostProcessor {
    pub function: Function,
    pub params: HashMap<String, String>,
}
