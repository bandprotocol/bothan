pub mod tick;

pub trait PostProcessor<E> {
    fn process(&self, data: f64) -> Result<f64, E>;
}
