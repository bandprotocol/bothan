use crate::registry::post_processor::PostProcess;
use crate::registry::processor::Process;
use crate::registry::source::SourceQuery;
use serde::{Deserialize, Serialize};

/// `Signal` contains the prerequisites, sources, processor, and post-processors for a signal.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Signal {
    #[serde(rename = "sources")]
    pub source_queries: Vec<SourceQuery>,
    pub processor: Process,
    pub post_processors: Vec<PostProcess>,
}
