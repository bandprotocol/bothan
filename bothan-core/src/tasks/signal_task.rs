use crate::registry::post_processor::{PostProcessor, PostProcessorError};
use crate::registry::processor::{Process, ProcessError};
use crate::registry::signal::Signal;
use rust_decimal::Decimal;

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
    pub fn execute_processor(&self, data: Vec<Decimal>) -> Result<Decimal, ProcessError> {
        self.signal.processor.process(data)
    }

    /// Executes and post processes the signal task given the processed data and returns out
    /// output. If the processing fails, it returns `None`.
    pub fn execute_post_processors(
        &self,
        processed: Decimal,
    ) -> Result<Decimal, PostProcessorError> {
        self.signal
            .post_processors
            .iter()
            .try_fold(processed, |acc, post| post.process(acc))
    }
}
