#![allow(dead_code)]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::post_processor::PostProcessor;
use crate::processor::Processor;
use crate::registry::source::Source;

pub mod source;

pub type Registry = HashMap<String, Signal>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Signal {
    pub prerequisites: Vec<String>,
    pub sources: Vec<Source>,
    pub processor: Processor,
    pub post_processors: Vec<PostProcessor>,
}
