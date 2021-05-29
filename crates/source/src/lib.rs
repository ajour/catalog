use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod tukui;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Flavor {
    Retail,
    RetailPtr,
    RetailBeta,
    ClassicEra,
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
    pub game_version: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Addon {
    pub id: u32,
    pub name: String,
    pub url: String,
    pub date_released: String,
    pub number_of_downloads: u32,
    pub summary: String,
    pub versions: Vec<Version>,
    pub categories: Vec<String>,
    pub source: String,
}
#[async_trait]
pub trait Source {
    async fn get_addons(&self, flavor: Flavor) -> Result<Vec<Addon>, Error>;
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Isahc(#[from] isahc::Error),
    #[error(transparent)]
    Http(#[from] isahc::http::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("unknown error")]
    Unknown,
}
