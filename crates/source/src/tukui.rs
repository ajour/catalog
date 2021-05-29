use super::*;
use async_trait::async_trait;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};

impl From<(Package, Flavor)> for Addon {
    fn from(pair: (Package, Flavor)) -> Self {
        let (package, flavor) = pair;
        Addon {
            id: package.id.parse().unwrap(), // TODO (casperstorm): handle this.
            name: package.name,
            url: package.web_url,
            date_released: package.last_download,
            number_of_downloads: package.downloads.parse().unwrap(), // TODO (casperstorm): handle this.
            summary: package.small_desc,
            versions: vec![Version {
                flavor,
                game_version: package.patch,
            }],
            categories: vec![package.category],
            source: "tuku".to_owned(),
        }
    }
}

// TODO (casperstorm): handle null deserialization.
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
    changelog: Option<String>,
}

pub struct Tukui {}

fn endpoint_for_flavor(flavor: &Flavor) -> String {
    let base_endpoint = "https://www.tukui.org/api.php";
    match flavor.base_flavor() {
        Flavor::Retail => {
            format!("{}?addons=all", base_endpoint)
        }
        Flavor::ClassicEra => {
            format!("{}?classic-addons=all", base_endpoint)
        }
        Flavor::ClassicTbc => {
            format!("{}?classic-tbc-addons=all", base_endpoint)
        }
        _ => panic!("Unknown base flavor {}", flavor),
    }
}

#[async_trait]
impl Source for Tukui {
    async fn get_addons(&self, flavor: Flavor) -> Result<Vec<Addon>, Error> {
        let mut response = isahc::get_async(endpoint_for_flavor(&flavor)).await?;
        let packages = response.json::<Vec<Package>>().await?;
        let addons = packages
            .into_iter()
            .map(|package| Addon::from((package, flavor)))
            .collect();
        Ok(addons)
    }
}
