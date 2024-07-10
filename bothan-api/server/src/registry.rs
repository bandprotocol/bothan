#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::post_processor::PostProcess;
use crate::processor::Process;
use crate::registry::source::Source;
use crate::tasks::utils::get_tasks;

pub mod source;

/// The `Registry` type is a HashMap that maps a signal name to its corresponding `Signal`.
pub type Registry = HashMap<String, Signal>;

/// `Signal` contains the prerequisites, sources, processor, and post-processors for a signal.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Signals {
    pub processor: Process,
    pub post_processors: Vec<PostProcess>,
    pub signals: Vec<Signals>,
}

/// `Signal` contains the prerequisites, sources, processor, and post-processors for a signal.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Signal {
    pub prerequisites: Vec<String>,
    pub sources: Vec<Source>,
}

pub trait Validator {
    fn validate(&self) -> bool;
}

impl Validator for Registry {
    fn validate(&self) -> bool {
        let validate_tasks = get_tasks(self).is_ok();
        let validate_source_routes = self.iter().all(|(_, signal)| {
            signal.sources.iter().all(|source| {
                source
                    .routes
                    .iter()
                    .all(|route| signal.prerequisites.contains(&route.signal_id))
            })
        });

        validate_tasks && validate_source_routes
    }
}
