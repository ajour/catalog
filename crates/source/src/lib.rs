use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod curse;
pub mod townlong_yak;
pub mod tukui;
pub mod wowinterface;

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
    pub source: String,
}

#[async_trait]
pub trait Source {
    async fn get_addons(&self) -> Result<Vec<Addon>, Error>;
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Isahc(#[from] isahc::Error),
    #[error(transparent)]
    Http(#[from] isahc::http::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("unknown error")]
    Unknown,
}

pub mod null_to_default {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: Default + Deserialize<'de>,
    {
        let opt = Option::deserialize(deserializer)?;
        Ok(opt.unwrap_or_default())
    }
}

pub mod number_and_string_to_i32 {
    use serde::{self, de, Deserialize, Deserializer};
    use std::convert::TryFrom;

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i32, D::Error> {
        Ok(match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::String(s) => s.parse().map_err(de::Error::custom)?,
            serde_json::Value::Number(num) => {
                let num = num
                    .as_i64()
                    .ok_or_else(|| de::Error::custom(format!("invalid number: {}", num)))?;
                i32::try_from(num).ok().unwrap_or(0)
            }
            _ => return Err(de::Error::custom("wrong type")),
        })
    }
}

pub mod number_and_string_to_u64 {
    use serde::{self, de, Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        Ok(match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::String(s) => s.parse().map_err(de::Error::custom)?,
            serde_json::Value::Number(num) => num
                .as_u64()
                .ok_or_else(|| de::Error::custom(format!("Invalid number: {}", num)))?,
            _ => return Err(de::Error::custom("wrong type")),
        })
    }
}

pub mod u64_to_string {
    use serde::{self, de, Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
        Ok(match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::Number(num) => {
                let num = num
                    .as_u64()
                    .ok_or_else(|| de::Error::custom(format!("invalid number: {}", num)))?;
                num.to_string()
            }
            _ => return Err(de::Error::custom("wrong type")),
        })
    }
}
