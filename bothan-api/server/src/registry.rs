#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::post_processor::PostProcess;
use crate::processor::Process;
use crate::registry::source::Source;

pub mod source;

/// The `Registry` type is a HashMap that maps a signal name to its corresponding `Signal`.
pub type Registry = HashMap<String, Signal>;

/// `Signal` contains the prerequisites, sources, processor, and post-processors for a signal.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Signal {
    pub prerequisites: Vec<String>,
    pub sources: Vec<Source>,
    pub processor: Process,
    pub post_processors: Vec<PostProcess>,
}
