use super::*;
pub struct Tukui {}

impl Source for Tukui {
    fn get_addons(&self) -> Result<Vec<Addon>, SourceError> {
        Ok(vec![])
    }
}
