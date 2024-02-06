#[derive(Debug)]
pub enum Error {
    DoesNotExist,
    Invalid,
    AlreadySet,
    PendingNotSet,
}
