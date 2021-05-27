use super::*;
use async_trait::async_trait;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Package {
    name: String,
}

impl From<Package> for Addon {
    fn from(item: Package) -> Self {
        Addon { name: item.name }
    }
}

pub struct Tukui {}

#[async_trait]
impl Source for Tukui {
    async fn get_addons(&self) -> Result<Vec<Addon>, SourceError> {
        let mut response =
            isahc::get_async("https://www.tukui.org/api.php?classic-tbc-addons=all").await?;
        let packages = response.json::<Vec<Package>>().await?;
        let addons = packages.into_iter().map(Addon::from).collect();
        Ok(addons)
    }
}
