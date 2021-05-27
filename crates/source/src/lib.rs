use async_trait::async_trait;

pub mod tukui;

#[derive(Clone, Debug)]
pub struct Addon {
    pub name: String,
}
#[async_trait]
pub trait Source {
    async fn get_addons(&self) -> Result<Vec<Addon>, SourceError>;
}

#[derive(thiserror::Error, Debug)]
pub enum SourceError {
    #[error("unknown error")]
    Unknown,
}
