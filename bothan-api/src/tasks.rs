#![allow(dead_code)]

use std::ops::Deref;

use crate::config::registry::Registry;
use crate::tasks::error::Error;
use crate::tasks::task::Task;
use crate::tasks::util::get_batched_tasks;

pub mod error;
pub mod task;
pub(crate) mod util;

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
