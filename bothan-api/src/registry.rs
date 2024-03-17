#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::post_processor::PostProcess;
use crate::processor::Process;
use crate::registry::source::Source;

pub mod source;

pub type Registry = HashMap<String, Signal>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Signal {
    pub prerequisites: Vec<String>,
    pub sources: Vec<Source>,
    pub processor: Process,
    pub post_processors: Vec<PostProcess>,
}
