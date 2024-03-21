use crate::post_processor::PostProcessor;
use crate::processor::Processor;
use crate::registry::Signal;

pub struct SignalTask {
    signal_id: String,
    signal: Signal,
}

impl SignalTask {
    pub fn new(signal_id: String, signal: Signal) -> Self {
        SignalTask { signal_id, signal }
    }

    pub fn execute(&self, data: Vec<f64>, prerequisites: Vec<f64>) -> Option<f64> {
        let processed: f64 = self.signal.processor.process(data, prerequisites).ok()?;
        self.signal
            .post_processors
            .iter()
            .try_fold(processed, |acc, post| post.process(acc).ok())
    }
}
