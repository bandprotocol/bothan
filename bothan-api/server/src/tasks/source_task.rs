use std::collections::HashSet;

/// A SourceTask represents a task that is responsible for fetching price data from a source. It
/// contains a `source_name` which is the name of the source and a set of `source_ids` which are the
/// ids to fetch the data from.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceTask {
    source_name: String,
    source_ids: HashSet<String>,
}

impl SourceTask {
    /// Creates a new `SourceTask` from a given `source_name` and a set of `source_ids`.
    pub fn new(source_name: String, source_ids: HashSet<String>) -> Self {
        SourceTask {
            source_name,
            source_ids,
        }
    }

    /// Returns the source name of the task.
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Returns the source ids of the task.
    pub fn source_ids(&self) -> Vec<&str> {
        self.source_ids.iter().map(|s| s.as_str()).collect()
    }
}
