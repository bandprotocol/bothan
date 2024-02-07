#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("symbol value does not exist")]
    DoesNotExist,

    #[error("symbol is invalid")]
    Invalid,

    #[error("symbol has already been set")]
    AlreadySet,

    #[error("symbol has not been set to pending")]
    PendingNotSet,
}
