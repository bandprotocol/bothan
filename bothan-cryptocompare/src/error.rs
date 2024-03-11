// TODO: Add more errors apart from catch all
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown error: {0}")]
    Unknown(String),

    #[error("pending result")]
    Pending,

    #[error("api error: {0}")]
    Api(#[from] crate::api::error::Error),
}
