use std::collections::HashMap;

pub struct Task {
    signal_ids: Vec<String>,
    source_tasks: HashMap<String, Vec<String>>,
    signal_ids_prerequisites: HashMap<String, Vec<String>>,
}

impl Task {
    pub fn new(
        signal_ids: Vec<String>,
        source_tasks: HashMap<String, Vec<String>>,
        signal_ids_prerequisites: HashMap<String, Vec<String>>,
    ) -> Self {
        Task {
            signal_ids,
            source_tasks,
            signal_ids_prerequisites,
        }
    }

    // Gets all the signal ids in the task
    pub fn get_signal_ids(&self) -> &Vec<String> {
        &self.signal_ids
    }

    // Gets the prerequisites for a given signal id. If the signal id is not found, it returns None
    pub fn get_signal_id_prerequisites(&self, id: &str) -> Option<&Vec<String>> {
        self.signal_ids_prerequisites.get(id)
    }

    // Get a mapping of the required ids for each source
    pub fn get_source_tasks(&self) -> &HashMap<String, Vec<String>> {
        &self.source_tasks
    }
}
