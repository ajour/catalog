use super::*;
use async_trait::async_trait;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};

impl From<(Package, Flavor)> for Addon {
    fn from(pair: (Package, Flavor)) -> Self {
        let (package, _flavor) = pair;
        Addon {
            id: package.id.parse().unwrap(),
            name: package.name,
        }
    }
}

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

pub struct Tukui {}

#[async_trait]
impl Source for Tukui {
    async fn get_addons(&self, flavor: Flavor) -> Result<Vec<Addon>, Error> {
        let mut response =
            isahc::get_async("https://www.tukui.org/api.php?classic-tbc-addons=all").await?;
        let packages = response.json::<Vec<Package>>().await?;
        let addons = packages
            .into_iter()
            .map(|package| Addon::from((package, flavor)))
            .collect();
        Ok(addons)
    }
}
