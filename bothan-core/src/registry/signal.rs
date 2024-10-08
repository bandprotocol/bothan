use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::registry::post_processor::PostProcessor;
use crate::registry::processor::Processor;
use crate::registry::source::SourceQuery;

/// `Signal` contains the sources, processor, and post-processors for a signal.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Signal {
    #[serde(rename = "sources")]
    pub source_queries: Vec<SourceQuery>,
    pub processor: Processor,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub post_processors: Vec<PostProcessor>,
}

impl Signal {
    /// Creates a new Signal.
    pub fn new(
        source_queries: Vec<SourceQuery>,
        processor: Processor,
        post_processors: Vec<PostProcessor>,
    ) -> Self {
        Signal {
            source_queries,
            processor,
            post_processors,
        }
    }
}
