#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Isahc(#[from] isahc::Error),
    #[error(transparent)]
    Http(#[from] isahc::http::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("unknown error")]
    Unknown,
}
