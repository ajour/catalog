use super::*;
use async_trait::async_trait;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct Package {
    id: i32,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    category_id: i32,
    version: String,
    last_update: u64,
    title: String,
    author: String,
    file_info_uri: String,
    downloads: u64,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    game_versions: Vec<String>,
}

fn base_endpoint() -> String {
    "https://api.mmoui.com/v4/game/WOW/filelist.json".to_owned()
}

pub struct WoWInterface {}

#[async_trait]
impl Source for WoWInterface {
    async fn get_addons(&self, flavor: Flavor) -> Result<Vec<Addon>, Error> {
        let mut response = isahc::get_async(base_endpoint()).await?;
        let packages = response.json::<Vec<Package>>().await?;
        println!("{:?}", packages);
        Ok(vec![])
    }
}
