use std::collections::HashMap;

use crate::registry::Signal;

pub struct Task {
    signals: HashMap<String, Signal>,
    source_tasks: HashMap<String, Vec<String>>,
}

impl Task {
    pub fn new(
        signals: HashMap<String, Signal>,
        source_tasks: HashMap<String, Vec<String>>,
    ) -> Self {
        Task {
            signals,
            source_tasks,
        }
    }

    // Gets all the signal ids in the task
    pub fn get_signal_ids(&self) -> Vec<&String> {
        self.signals.keys().collect()
    }

    pub fn get_signals(&self) -> Vec<(&String, &Signal)> {
        self.signals.iter().collect()
    }

    // Get a mapping of the required ids for each source
    pub fn get_source_tasks(&self) -> Vec<(&String, &Vec<String>)> {
        self.source_tasks.iter().collect()
    }
}
