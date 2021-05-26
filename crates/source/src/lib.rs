pub mod tukui;

#[derive(Clone, Debug)]
pub struct Addon {
    pub name: String,
}
pub trait Source {
    fn get_addons(&self) -> Result<Vec<Addon>, SourceError>;
}

#[derive(thiserror::Error, Debug)]
pub enum SourceError {
    #[error("unknown error")]
    Unknown,
}
