use super::*;
use async_trait::async_trait;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Package {
    id: String,
    name: String,
    small_desc: String,
    author: String,
    version: String,
    screenshot_url: String,
    url: String,
    category: String,
    downloads: String,
    lastupdate: String,
    patch: String,
    web_url: String,
    last_download: String,
    // changelog: String,
}

impl From<Package> for Addon {
    fn from(item: Package) -> Self {
        Addon {
            id: item.id.parse().unwrap(),
            name: item.name,
        }
    }
}

pub struct Tukui {}

#[async_trait]
impl Source for Tukui {
    async fn get_addons(&self, flavor: Flavor) -> Result<Vec<Addon>, Error> {
        let mut response =
            isahc::get_async("https://www.tukui.org/api.php?classic-tbc-addons=all").await?;
        let packages = response.json::<Vec<Package>>().await?;
        let addons = packages.into_iter().map(Addon::from).collect();
        Ok(addons)
    }
}
