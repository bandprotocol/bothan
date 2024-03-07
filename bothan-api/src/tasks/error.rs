#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cycle detected error")]
    CycleDetected(),
}

impl<N> From<petgraph::algo::Cycle<N>> for Error {
    fn from(_: petgraph::algo::Cycle<N>) -> Self {
        Error::CycleDetected()
    }
}
