use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod tukui;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Addon {
    pub name: String,
}
#[async_trait]
pub trait Source {
    async fn get_addons(&self) -> Result<Vec<Addon>, Error>;
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Isahc(#[from] isahc::Error),
    #[error(transparent)]
    Http(#[from] isahc::http::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("unknown error")]
    Unknown,
}
