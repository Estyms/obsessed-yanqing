use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::data::core::{PrydwenCompatible, PrydwenResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrydwenCone {
    pub all_characters: AllCharacters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllCharacters {
    pub nodes: Vec<Cone>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cone {
    pub id: String,
    pub updated_at: String,
    pub created_at: String,
    pub cone_id: i64,
    pub name: String,
    pub slug: String,
    pub image: Option<Image>,
    pub small_image: SmallImage,
    pub rarity: String,
    #[serde(default)]
    pub source: Option<Vec<String>>,
    pub release_date: String,
    pub path: String,
    pub stats: Stats,
    pub character_builder_cone_info: CharacterBuilderConeInfo,
    pub skill_name: String,
    pub skill_description: SkillDescription,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub local_file: LocalFile,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalFile {
    pub child_image_sharp: ChildImageSharp,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildImageSharp {
    pub gatsby_image_data: GatsbyImageData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatsbyImageData {
    pub layout: String,
    pub background_color: String,
    pub images: Images,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Images {
    pub fallback: Fallback,
    pub sources: Vec<Source>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fallback {
    pub src: String,
    pub src_set: String,
    pub sizes: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub src_set: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallImage {
    pub local_file: LocalFile2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalFile2 {
    pub child_image_sharp: ChildImageSharp2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildImageSharp2 {
    pub gatsby_image_data: GatsbyImageData2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatsbyImageData2 {
    pub layout: String,
    pub background_color: String,
    pub images: Images2,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Images2 {
    pub fallback: Fallback2,
    pub sources: Vec<Source2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fallback2 {
    pub src: String,
    pub src_set: String,
    pub sizes: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source2 {
    pub src_set: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sizes: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub hp: Hp,
    pub atk: Atk,
    pub def: Def,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hp {
    #[serde(rename = "value_level_1")]
    pub value_level_1: String,
    #[serde(rename = "value_level_max")]
    pub value_level_max: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Atk {
    #[serde(rename = "value_level_1")]
    pub value_level_1: String,
    #[serde(rename = "value_level_max")]
    pub value_level_max: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Def {
    #[serde(rename = "value_level_1")]
    pub value_level_1: String,
    #[serde(rename = "value_level_max")]
    pub value_level_max: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterBuilderConeInfo {
    pub cone_custom_stat1: ConeCustomStat1,
    pub cone_custom_stat2: ConeCustomStat2,
    pub cone_custom_stat3: ConeCustomStat3,
    pub cone_custom_stat4: ConeCustomStat4,
    pub comment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConeCustomStat1 {
    pub stat: String,
    #[serde(rename = "value_1")]
    pub value_1: i64,
    #[serde(rename = "value_5")]
    pub value_5: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConeCustomStat2 {
    pub stat: String,
    #[serde(rename = "value_1")]
    pub value_1: i64,
    #[serde(rename = "value_5")]
    pub value_5: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConeCustomStat3 {
    pub stat: String,
    #[serde(rename = "value_1")]
    pub value_1: i64,
    #[serde(rename = "value_5")]
    pub value_5: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConeCustomStat4 {
    pub stat: String,
    #[serde(rename = "value_1")]
    pub value_1: i64,
    #[serde(rename = "value_5")]
    pub value_5: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillDescription {
    pub raw: String,
}

impl PrydwenCompatible for PrydwenCone {}

pub async fn get_light_cone(name: String) -> Option<Cone> {
    let data_response = reqwest::get("https://www.prydwen.gg/page-data/star-rail/light-cones/page-data.json").await.ok()?;
    let d = data_response.text().await.ok()?;
    let js = &mut serde_json::Deserializer::from_str(d.as_str());
    let data : Result<PrydwenResponse<PrydwenCone>, _> = serde_path_to_error::deserialize(js);
    match data {
        Ok(d) => {
            Some(d.result.data.all_characters.nodes.into_iter().filter(|f| f.slug.eq(name.as_str())).collect::<Vec<Cone>>().get(0).expect(format!("Cannot find {}", name).as_str()).clone())
        }
        Err(err) => {
            let path = err.path().to_string();
            println!("{}", path);
            None
        }
    }
}