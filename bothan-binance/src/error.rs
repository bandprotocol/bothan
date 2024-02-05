// TODO: Add more errors apart from catch all
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("pending result")]
    Pending,

    #[error("tungstenite error")]
    TungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),
}
