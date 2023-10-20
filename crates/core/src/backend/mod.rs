use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub mod curse;
pub mod hub;
pub mod tukui;
pub mod wowinterface;

#[async_trait]
pub trait Backend {
    async fn get_addons(&self) -> Result<Vec<Addon>, Error>;
}

#[async_trait]
impl Backend for Source {
    async fn get_addons(&self) -> Result<Vec<Addon>, Error> {
        match self {
            Source::Curse => curse::get_addons().await,
            Source::Tukui => tukui::get_addons().await,
            Source::WowI => wowinterface::get_addons().await,
            Source::Hub => hub::get_addons().await,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Source {
    Curse,
    Tukui,
    WowI,
    Hub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Ord)]
pub enum Flavor {
    #[serde(alias = "retail", alias = "wow_retail", alias = "mainline")]
    Retail,
    RetailPtr,
    RetailBeta,
    #[serde(
        alias = "classic",
        alias = "wow_classic",
        alias = "classic_era",
        alias = "vanilla"
    )]
    ClassicEra,
    #[serde(
        alias = "tbc",
        alias = "bcc",
        alias = "wow_burning_crusade",
        alias = "burningCrusade",
        alias = "burning_crusade"
    )]
    ClassicTbc,
    #[serde(alias = "wotlk")]
    ClassicWotlk,
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
                Flavor::ClassicWotlk => "classic_wotlk",
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
            Flavor::ClassicWotlk | Flavor::ClassicPtr | Flavor::ClassicBeta => Flavor::ClassicWotlk,
            Flavor::ClassicTbc => Flavor::ClassicTbc,
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
