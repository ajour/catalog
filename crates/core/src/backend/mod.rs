use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub mod curse;
pub mod townlong_yak;
pub mod tukui;
pub mod wowinterface;

#[async_trait]
pub trait Backend {
    async fn get_addons(&self) -> Result<Vec<Addon>, Error>;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Source {
    Curse,
    Tukui,
    WowI,
    TownlongYak,
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Source::Curse => "Curse",
            Source::Tukui => "Tukui",
            Source::WowI => "WowInterface",
            Source::TownlongYak => "TownlongYak",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Ord)]
pub enum Flavor {
    #[serde(alias = "retail", alias = "wow_retail")]
    Retail,
    RetailPtr,
    RetailBeta,
    #[serde(alias = "classic", alias = "wow_classic", alias = "classic_era")]
    ClassicEra,
    #[serde(
        alias = "wow_burning_crusade",
        alias = "burningCrusade",
        alias = "burning_crusade"
    )]
    ClassicTbc,
    ClassicPtr,
    ClassicBeta,
}

impl std::fmt::Display for Flavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Flavor::Retail => "retail",
                Flavor::RetailPtr => "retail_ptr",
                Flavor::RetailBeta => "retail_beta",
                Flavor::ClassicEra => "classic_era",
                Flavor::ClassicTbc => "classic_tbc",
                Flavor::ClassicBeta => "classic_beta",
                Flavor::ClassicPtr => "classic_ptr",
            }
        )
    }
}

impl Flavor {
    /// Returns `Flavor` which self relates to.
    pub fn base_flavor(self) -> Flavor {
        match self {
            Flavor::Retail | Flavor::RetailPtr | Flavor::RetailBeta => Flavor::Retail,
            Flavor::ClassicTbc | Flavor::ClassicPtr | Flavor::ClassicBeta => Flavor::ClassicTbc,
            Flavor::ClassicEra => Flavor::ClassicEra,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Version {
    pub flavor: Flavor,
    pub game_version: Option<String>,
    pub date: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Addon {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub number_of_downloads: u64,
    pub summary: String,
    pub versions: Vec<Version>,
    pub categories: Vec<String>,
    pub source: Source,
}
