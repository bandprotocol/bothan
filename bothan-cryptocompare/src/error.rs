// TODO: Add more errors apart from catch all
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("api error: {0}")]
    Api(#[from] crate::api::error::Error),
}
