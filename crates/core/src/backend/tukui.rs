use futures::try_join;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend::{Addon, Flavor, Source, Version};
use crate::error::Error;
use crate::utility::{null_to_default, number_and_string_to_i32, number_and_string_to_u64};

impl From<(Package, Flavor)> for Addon {
    fn from(pair: (Package, Flavor)) -> Self {
        let (package, flavor) = pair;
        Addon {
            id: package.id,
            name: package.name,
            url: package.web_url,
            number_of_downloads: package.downloads,
            summary: package.small_desc,
            versions: vec![Version {
                flavor,
                game_version: Some(package.patch),
                date: package.lastupdate,
            }],
            categories: vec![package.category],
            source: Source::Tukui,
        }
    }
}

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

fn base_endpoint<'a>() -> &'a str {
    "https://www.tukui.org/api.php"
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
        Flavor::ClassicWotlk => {
            format!("{}?classic-wotlk-addons=all", base_endpoint)
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

pub async fn get_addons() -> Result<Vec<Addon>, Error> {
    let flavors = vec![Flavor::Retail, Flavor::ClassicEra, Flavor::ClassicTbc, Flavor::ClassicWotlk];
    let mut addons: Vec<Addon> = vec![];
    for flavor in flavors.iter() {
        match flavor.base_flavor() {
            // When fetching retail AddOns, we have to get the two main addons;
            // Elvui & Tukui from two seperate endpoints, and then combine with
            // the rest.
            Flavor::Retail => {
                let elv_res_future = isahc::get_async(endpoint_for_elvui());
                let tuk_res_future = isahc::get_async(endpoint_for_tukui());
                let all_res_future = isahc::get_async(endpoint_for_addons(&flavor));

                let (mut elv_res, mut tuk_res, mut all_res) =
                    try_join!(elv_res_future, tuk_res_future, all_res_future)?;

                let elv_json_future = elv_res.json::<Package>();
                let tuk_json_future = tuk_res.json::<Package>();
                let all_json_future = all_res.json::<Vec<Package>>();

                let (elv_package, tuk_package, all_packages) =
                    try_join!(elv_json_future, tuk_json_future, all_json_future)?;

                let mut packages = vec![];
                packages.extend(all_packages);
                packages.push(elv_package);
                packages.push(tuk_package);

                // Extends addons with `Package` converted to `Addon`.
                addons.extend(
                    packages
                        .into_iter()
                        .map(|package| Addon::from((package, *flavor)))
                        .collect::<Vec<Addon>>(),
                );
            }
            _ => {
                let mut response = isahc::get_async(endpoint_for_addons(&flavor)).await?;
                let packages = response.json::<Vec<Package>>().await?;

                // Extends addons with `Package` converted to `Addon`.
                addons.extend(
                    packages
                        .into_iter()
                        .map(|package| Addon::from((package, *flavor)))
                        .collect::<Vec<Addon>>(),
                );
            }
        }
    }

    Ok(addons)
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
