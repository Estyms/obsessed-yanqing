use std::cmp::Ordering;
use levenshtein::levenshtein;
use crate::data::core::{PrydwenCompatible, PrydwenResponse};
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrydwenAllCharacters {
    pub all_characters: AllCharacters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllCharacters {
    pub nodes: Vec<Characters>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Characters {
    pub id: String,
    pub unit_id: i64,
    pub slug: String,
    pub name: String,
    pub rarity: String,
    pub element: String,
    pub path: String,
}


impl PrydwenCompatible for PrydwenAllCharacters{}

pub async fn get_nearest_characters(name: String) -> Option<Vec<Characters>> {
    let mut characters = get_all_characters().await.expect("Cannot get characters");
    characters.nodes.sort_by(|a, b | {
        match a.name.to_lowercase().contains(name.to_lowercase().as_str()) {
            true => {
                match b.name.to_lowercase().contains(name.to_lowercase().as_str()) {
                    true => Ordering::Equal,
                    false => Ordering::Less
                }
            }
            false => {
                match b.name.to_lowercase().contains(name.to_lowercase().as_str()){
                    true => Ordering::Greater,
                    false => levenshtein(a.name.to_lowercase().as_str(), name.to_lowercase().as_str()).partial_cmp(&levenshtein(b.name.to_lowercase().as_str(), name.to_lowercase().as_str())).unwrap()
                }
            }
        }
    });
    characters.nodes.truncate(5);
    Some(characters.nodes)
}

async fn get_all_characters() -> Option<AllCharacters> {
    let data = reqwest::get("https://www.prydwen.gg/page-data/star-rail/characters/page-data.json").await.ok()?.json::<PrydwenResponse<PrydwenAllCharacters>>().await;
    match data {
        Ok(d) => {
            Some(d.result.data.all_characters)
        }
        Err(_) => None
    }
}