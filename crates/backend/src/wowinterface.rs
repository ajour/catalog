use super::*;
use async_trait::async_trait;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};

impl From<Package> for Addon {
    fn from(package: Package) -> Self {
        let categories =
            category_name_for_category_id(package.category_id).map_or(vec![], |c| vec![c]);
        let flavor = flavor_for_category_id(package.category_id);
        let version = extract_version_for_flavor(flavor, package.game_versions);

        Addon {
            id: package.id,
            name: package.title,
            url: package.file_info_uri,
            number_of_downloads: package.downloads,
            // Currently API does not send any description.
            summary: "".to_owned(),
            versions: vec![Version {
                flavor,
                game_version: version,
                date: package.last_update,
            }],
            categories,
            source: Source::WowI,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct Package {
    id: i32,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    category_id: i32,
    version: String,
    #[serde(deserialize_with = "u64_to_string::deserialize")]
    last_update: String,
    title: String,
    author: String,
    file_info_uri: String,
    downloads: u64,
    #[serde(deserialize_with = "null_to_default::deserialize")]
    game_versions: Vec<String>,
}

fn base_endpoint<'a>() -> &'a str {
    "https://api.mmoui.com/v4/game/WOW/filelist.json"
}

/// Returns `Flavor` for a category id `i32`.
/// WoWInterface has multiple categories. Classic and TBC has its own
/// category with addons.
fn flavor_for_category_id(id: i32) -> Flavor {
    match id {
        160 => Flavor::ClassicEra,
        161 => Flavor::ClassicTbc,
        _ => Flavor::Retail,
    }
}

/// Trys to guess `Flavor` from version `String`.
///
/// Eg. `"9.0.5" => Flavor::Retail`
fn guess_flavor_from_version(version: &str) -> Option<Flavor> {
    match version.chars().next() {
        // 9.x.x, 8.x.x, 7.x.x
        Some('9' | '8' | '7') => Some(Flavor::Retail),
        // 1.x.x
        Some('1') => Some(Flavor::ClassicEra),
        // 2.x.x
        Some('2') => Some(Flavor::ClassicTbc),
        _ => None,
    }
}

/// Extract version from `Vec<String>` for specific `Flavor`.
///
/// Versions looks like: `["2.5.1", "9.0.5", "1.13.7"]`.
/// Eg. for `Flavor::Retail` we want to extract `9.0.5`.
fn extract_version_for_flavor(flavor: Flavor, versions: Vec<String>) -> Option<String> {
    for version in versions {
        let flavor_guess = guess_flavor_from_version(&version);
        if flavor_guess == Some(flavor) {
            return Some(version);
        }
    }

    None
}

/// Returns category name as `String` for given category id `i32`.
fn category_name_for_category_id(id: i32) -> Option<String> {
    let category_name = match id {
        17 => "Graphic UI Mods",
        18 => "Character Advancement",
        19 => "Action Bar Mods",
        20 => "Bags, Bank, Inventory",
        21 => "Unit Mods",
        22 => "Buff, Debuff, Spell",
        24 => "Map, Coords, Compasses",
        25 => "Combat Mods",
        26 => "Data Mods",
        27 => "Miscellaneous",
        35 => "Developer Utilities",
        39 => "Class & Role Specific",
        40 => "TradeSkill Mods",
        45 => "Raid Mods",
        53 => "Libraries",
        55 => "Chat Mods",
        56 => "Druid",
        57 => "Hunter",
        58 => "Mage",
        59 => "Paladin",
        60 => "Priest",
        61 => "Rogue",
        62 => "Shaman",
        63 => "Warlock",
        64 => "Warrior",
        85 => "FuBar",
        88 => "WoW Tools & Utilities",
        94 => "Auction House & Vendors",
        95 => "Group, Guild & Friends",
        96 => "PvP, Arena, BattleGrounds",
        97 => "Mail",
        98 => "ToolTip",
        99 => "Titan Panel",
        100 => "Mini Games, ROFL",
        108 => "Data Broker",
        109 => "Info, Plug-in Bars",
        111 => "Other",
        112 => "Casting Bars, Cooldowns",
        113 => "Suites",
        114 => "RolePlay, Music Mods",
        146 => "Mounts & Pets",
        147 => "UI Media",
        149 => "DPS",
        150 => "Healers",
        151 => "Death Knight",
        152 => "Monk",
        153 => "Tanks",
        154 => "Utility Mods",
        155 => "Garrisons",
        157 => "Demon Hunter",
        160 => "Classic",
        161 => "The Burning Crusade Classic",
        _ => "",
    };

    if category_name.is_empty() {
        None
    } else {
        Some(category_name.to_owned())
    }
}

pub struct WoWInterface {}

#[async_trait]
impl Backend for WoWInterface {
    async fn get_addons(&self) -> Result<Vec<Addon>, Error> {
        let mut response = isahc::get_async(base_endpoint()).await?;
        let packages = response.json::<Vec<Package>>().await?;
        let addons = packages
            .into_iter()
            .map(Addon::from)
            .collect::<Vec<Addon>>();
        Ok(addons)
    }
}

#[test]
fn test_null_fields() {
    let tests = [
        r"[]",
        r#"[{
            "id": 38,
            "categoryId": null,
            "version": "9.0.5.7",
            "lastUpdate": 1622059572000,
            "title": "Foo",
            "author": "Bar",
            "fileInfoUri": "",
            "downloads": 593294,
            "downloadsMonthly": 128,
            "favorites": 689,
            "gameVersions": [],
            "checksum": "",
            "addons": null
        }]"#,
    ];

    for test in tests.iter() {
        serde_json::from_str::<Vec<Package>>(test).unwrap();
    }
}
