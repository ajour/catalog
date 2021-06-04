use super::*;
use async_trait::async_trait;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};

// TODO (casperstorm):
// 1. tukui and elvui addons is located at for retail:
// endpoint?ui=elvui"
// endpoint?ui=tukui"
// This is currently not handled.

impl From<(Package, Flavor)> for Addon {
    fn from(pair: (Package, Flavor)) -> Self {
        let (package, flavor) = pair;
        Addon {
            id: package.id,
            name: package.name,
            url: package.web_url,
            date_released: package.lastupdate,
            number_of_downloads: package.downloads,
            summary: package.small_desc,
            versions: vec![Version {
                flavor,
                game_version: package.patch,
            }],
            categories: vec![package.category],
            source: "tukui".to_owned(),
        }
    }
}

// TODO (casperstorm): handle null deserialization.
#[derive(Deserialize, Serialize, Clone, Debug)]
struct Package {
    #[serde(deserialize_with = "number_and_string_to_i32::deserialize")]
    id: i32,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    name: String,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    small_desc: String,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    author: String,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    version: String,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    screenshot_url: String,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    url: String,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    category: String,
    #[serde(deserialize_with = "number_and_string_to_u64::deserialize")]
    downloads: u64,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    lastupdate: String,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    patch: String,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    web_url: String,
}

pub struct Tukui {}

fn base_endpoint() -> String {
    "https://www.tukui.org/api.php".to_owned()
}

fn endpoint_for_addons(flavor: &Flavor) -> String {
    let base_endpoint = base_endpoint();
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

fn endpoint_for_tukui() -> String {
    format!("{}?ui=tukui", base_endpoint())
}

fn endpoint_for_elvui() -> String {
    format!("{}?ui=elvui", base_endpoint())
}

#[async_trait]
impl Source for Tukui {
    async fn get_addons(&self, flavor: Flavor) -> Result<Vec<Addon>, Error> {
        let packages = match flavor.base_flavor() {
            Flavor::Retail => {
                let mut response = isahc::get_async(endpoint_for_elvui()).await?;
                let package = response.json::<Package>().await?;
                let mut packages = Vec::new();
                packages.push(package);

                packages
                // let mut response = isahc::get_async(endpoint_for_addons(&flavor)).await?;
                // response.json::<Vec<Package>>().await?
            }
            _ => {
                let mut response = isahc::get_async(endpoint_for_addons(&flavor)).await?;
                response.json::<Vec<Package>>().await?
            }
        };
        let addons = packages
            .into_iter()
            .map(|package| Addon::from((package, flavor)))
            .collect();
        Ok(addons)
    }
}

#[test]
fn test_null_fields() {
    let tests = [
        r"[]",
        r#"[{
            "id": null,
            "name": null,
            "small_desc": null,
            "author": null,
            "version": null,
            "screenshot_url": null,
            "web_url": null,
            "url": null,
            "category": null,
            "downloads": null,
            "lastupdate": null,
            "patch": null,
            "last_download": null
        }]"#,
    ];

    for test in tests.iter() {
        serde_json::from_str::<Vec<Package>>(test).unwrap();
    }
}
