pub mod tukui;

pub(crate) struct Addon {}
pub(crate) trait Source {
    fn get_addons(&self) -> Result<Vec<Addon>, SourceError>;
}

#[derive(thiserror::Error, Debug)]
pub enum SourceError {
    #[error("unknown error")]
    Unknown,
}
