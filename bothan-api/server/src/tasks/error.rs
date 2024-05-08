#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cycle detected error")]
    CycleDetected(),

    #[error("missing node")]
    MissingNode(),
}
