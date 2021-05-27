use super::*;
use async_trait::async_trait;
use isahc::prelude::*;
pub struct Tukui {}

#[async_trait]
impl Source for Tukui {
    async fn get_addons(&self) -> Result<Vec<Addon>, SourceError> {
        // Convert isahc error -> sourceerror.
        let response =
            isahc::get_async("https://www.tukui.org/api.php?classic-tbc-addons=all").await;
        println!("{:?}", response.unwrap().text().await);
        Ok(vec![])
    }
}
