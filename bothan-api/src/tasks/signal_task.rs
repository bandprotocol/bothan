use crate::post_processor::PostProcessor;
use crate::processor::Processor;
use crate::registry::Signal;

/// `SignalTask` represents the tasks to processes a signal.
/// It contains a `signal_id` which is a unique identifier for the signal value and `signal`,
/// which contains the relevant signal information to be processed.
#[derive(Debug, Clone, PartialEq)]
pub struct SignalTask {
    signal_id: String,
    signal: Signal,
}

impl SignalTask {
    /// Creates a new `SignalTask` from a given `signal_id` and its corresponding `signal`.
    pub fn new(signal_id: String, signal: Signal) -> Self {
        SignalTask { signal_id, signal }
    }

    /// Returns the signal id.
    pub fn signal_id(&self) -> &str {
        &self.signal_id
    }

    /// Returns the signal of the signal task.
    pub fn signal(&self) -> &Signal {
        &self.signal
    }

    /// Executes and processes the signal task given the data and prerequisites and returns out
    /// output. If the processing fails, it returns `None`.
    pub fn execute(&self, data: Vec<f64>, prerequisites: Vec<f64>) -> Option<f64> {
        let processed: f64 = self.signal.processor.process(data, prerequisites).ok()?;
        self.signal
            .post_processors
            .iter()
            .try_fold(processed, |acc, post| post.process(acc).ok())
    }
}
