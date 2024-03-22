#![allow(dead_code)]
use crate::registry::Registry;
use crate::tasks::error::Error;
use crate::tasks::signal_task::SignalTask;
use crate::tasks::source_task::SourceTask;
use crate::tasks::utils::get_tasks;

pub mod error;
pub mod signal_task;
pub mod source_task;
pub(crate) mod utils;

pub struct Tasks {
    source_tasks: Vec<SourceTask>,
    batched_signal_tasks: Vec<Vec<SignalTask>>,
}

impl Tasks {
    pub fn new(source_tasks: Vec<SourceTask>, batched_signal_tasks: Vec<Vec<SignalTask>>) -> Self {
        Tasks {
            source_tasks,
            batched_signal_tasks,
        }
    }

    pub fn from_registry(registry: &Registry) -> Result<Self, Error> {
        let (batched_signal_tasks, source_tasks) = get_tasks(registry)?;
        Ok(Tasks::new(source_tasks, batched_signal_tasks))
    }

    pub fn source_tasks(&self) -> &Vec<SourceTask> {
        &self.source_tasks
    }

    pub fn batched_signal_tasks(&self) -> &Vec<Vec<SignalTask>> {
        &self.batched_signal_tasks
    }
}
