#[allow(unused_imports)]
use std::collections::HashMap;

use crate::registry::post_processor::PostProcessor;
use serde::{Deserialize, Serialize};

use crate::registry::processor::Processor;
use crate::registry::source::Source;

mod post_processor;
mod processor;
mod source;

pub type Registry = HashMap<String, Signal>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Signal {
    pub prerequisites: Vec<String>,
    pub sources: Vec<Source>,
    pub processors: Processor,
    pub post_processors: Vec<PostProcessor>,
}
