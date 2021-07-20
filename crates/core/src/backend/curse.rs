use async_trait::async_trait;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend::{Addon, Backend, Flavor, Source, Version};
use crate::error::Error;

impl From<File> for Version {
    fn from(file: File) -> Self {
        Version {
            game_version: file.game_version.first().cloned(),
            flavor: file.game_version_flavor,
            date: file.file_date,
        }
    }
}

impl From<Package> for Addon {
    fn from(package: Package) -> Self {
        let versions = package
            .latest_files
            .into_iter()
            .filter(|f| f.release_type == 1)
            .map(Version::from)
            .collect();
        Addon {
            id: package.id,
            name: package.name,
            url: package.website_url,
            number_of_downloads: package.download_count.round() as u64,
            summary: package.summary,
            versions,
            categories: package.categories.into_iter().map(|c| c.name).collect(),
            source: Source::Curse,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Category {
    name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct File {
    file_date: String,
    game_version_flavor: Flavor,
    game_version: Vec<String>,
    release_type: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct Package {
    id: i32,
    name: String,
    // TODO (casperstorm): Strangely enough this expect to be `f64`.
    // If set to u64, we get the following error:
    // "invalid type: floating point `338355764`, expected u64"
    download_count: f64,
    summary: String,
    website_url: String,
    categories: Vec<Category>,
    latest_files: Vec<File>,
}

pub struct Curse {}

fn base_endpoint(page_size: usize, index: usize) -> String {
    format!(
        "https://addons-ecs.forgesvc.net/api/v2/addon/search?gameId=1&pageSize={}&index={}",
        page_size, index
    )
}

#[async_trait]
impl Backend for Curse {
    async fn get_addons(&self) -> Result<Vec<Addon>, Error> {
        let mut index: usize = 0;
        let page_size: usize = 50;
        let mut number_of_addons = page_size;
        let mut addons: Vec<Addon> = vec![];
        while page_size == number_of_addons {
            let endpoint = base_endpoint(page_size, index);
            let mut response = isahc::get_async(endpoint).await?;
            let packages = response.json::<Vec<Package>>().await?;
            let partials_addons = packages
                .into_iter()
                .map(Addon::from)
                .collect::<Vec<Addon>>();

            addons.extend_from_slice(&partials_addons);
            number_of_addons = partials_addons.len();
            index += page_size;
        }

        Ok(addons)
    }
}
