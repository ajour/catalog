use async_trait::async_trait;
use isahc::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::backend::{Addon, Backend, Flavor, Source, Version};
use crate::error::Error;

impl From<(GameVersion, String)> for Version {
    fn from(pair: (GameVersion, String)) -> Self {
        let (game_version, date) = pair;
        Version {
            flavor: game_version.game_type,
            game_version: Some(game_version.interface),
            date,
        }
    }
}

impl From<Package> for Addon {
    fn from(package: Package) -> Self {
        let re = Regex::new(r"<[^>]*>").unwrap();
        let summary = re.replace_all(&package.description, "").to_string();
        let first_release = package.releases.first();
        let versions: Vec<Version> = if let Some(release) = first_release {
            release
                .clone()
                .game_versions
                .into_iter()
                .map(|gv| Version::from((gv, release.published_at.clone())))
                .collect::<Vec<Version>>()
        } else {
            vec![]
        };
        Addon {
            id: package.id,
            name: package.repository_name,
            url: package.repository,
            number_of_downloads: package.total_download_count,
            summary,
            versions,
            categories: vec![],
            source: Source::TownlongYak,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Container {
    addons: Vec<Package>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct GameVersion {
    game_type: Flavor,
    title: String,
    interface: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Release {
    // TODO (casperstorm): Uniform all dates across sources.
    // 2021-04-26T22:42:55.958Z
    published_at: String,
    game_versions: Vec<GameVersion>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Package {
    id: i32,
    repository: String,
    repository_name: String,
    source: String,
    description: String,
    homepage: String,
    image_url: String,
    owner_image_url: String,
    owner_name: String,
    total_download_count: u64,
    funding_links: Vec<String>,
    releases: Vec<Release>,
}

pub struct TownlongYak {}

fn base_endpoint<'a>() -> &'a str {
    "https://hub.wowup.io/addons/author/foxlit"
}

#[async_trait]
impl Backend for TownlongYak {
    async fn get_addons(&self) -> Result<Vec<Addon>, Error> {
        let mut response = isahc::get_async(base_endpoint()).await?;
        let container = response.json::<Container>().await?;
        let addons = container
            .addons
            .into_iter()
            .map(Addon::from)
            .collect::<Vec<Addon>>();
        Ok(addons)
    }
}
