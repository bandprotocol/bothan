use std::collections::HashMap;
use std::ops::Deref;

use crate::config::registry::Registry;
use crate::tasks::error::Error;
use crate::tasks::util::get_batched_tasks;

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

pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Tasks {
    pub fn new(tasks: Vec<Task>) -> Self {
        Tasks { tasks }
    }

    pub fn from_registry(registry: &Registry) -> Result<Self, Error> {
        let tasks = get_batched_tasks(registry)?
            .into_iter()
            .map(|(signal_ids, source_tasks)| {
                let prerequisites = signal_ids
                    .iter()
                    .map(|id| {
                        let prerequisite_ids = registry
                            .get(id)
                            .and_then(|v| v.prerequisites.clone())
                            .unwrap_or_default();
                        (id.clone(), prerequisite_ids)
                    })
                    .collect();

                let vectorized_source_task = source_tasks
                    .into_iter()
                    .map(|(k, v)| (k, v.into_iter().collect()))
                    .collect();

                Task::new(signal_ids, vectorized_source_task, prerequisites)
            })
            .collect();

        Ok(Tasks::new(tasks))
    }
}

impl Deref for Tasks {
    type Target = Vec<Task>;

    fn deref(&self) -> &Self::Target {
        &self.tasks
    }
}
