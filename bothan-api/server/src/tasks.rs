use crate::registry::Registry;
use crate::tasks::error::Error;
use crate::tasks::signal_task::SignalTask;
use crate::tasks::source_task::SourceTask;
use crate::tasks::utils::get_tasks;

pub mod error;
pub mod signal_task;
pub mod source_task;
mod utils;

/// Tasks contain the tasks to be executed. It is represented by a vector of `SourceTask` and a
/// vector `BatchedSignalTask` which contains SignalTask.
///
/// The tasks can be represented as:
///
/// `source_tasks` = [SourceTask1, SourceTask2, ...]
/// `batched_signal_tasks` = [[SignalTask1, SignalTask2, ...], [SignalTask3, SignalTask4, ...], ...]
///
/// where given the following example:
/// `source_tasks` = [SourceTask1, SourceTask2]
/// `batched_signal_tasks` = [[SignalTask1, SignalTask2], [SignalTask3, SignalTask4]]
///
/// the execution order would be:
///
/// 1. SourceTask1, SourceTask2
/// 2. SignalTask1, SignalTask2
/// 3. SignalTask3, SignalTask4
#[derive(Debug, Clone, PartialEq)]
pub struct Tasks {
    source_tasks: Vec<SourceTask>,
    batched_signal_tasks: Vec<SignalTask>,
}

impl Tasks {
    /// Creates a new `Tasks` given a vector of `SourceTask` and a vector of `BatchedSignalTask`.
    pub fn new(source_tasks: Vec<SourceTask>, batched_signal_tasks: Vec<SignalTask>) -> Self {
        Tasks {
            source_tasks,
            batched_signal_tasks,
        }
    }

    /// Splits the task into its constituent parts.
    pub fn split(self) -> (Vec<SourceTask>, Vec<SignalTask>) {
        (self.source_tasks, self.batched_signal_tasks)
    }
}

impl TryFrom<Registry> for Tasks {
    type Error = Error;

    fn try_from(registry: Registry) -> Result<Self, Self::Error> {
        let (signal_tasks, source_tasks) = get_tasks(&registry)?;
        Ok(Tasks::new(source_tasks, signal_tasks))
    }
}
