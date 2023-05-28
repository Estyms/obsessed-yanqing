use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use crate::data::core::{PrydwenCompatible, PrydwenResponse};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrydwenCharacter {
    pub current_unit: CharacterWrapper,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterWrapper {
    pub nodes: Vec<Character>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: String,
    pub updated_at: String,
    pub created_at: String,
    pub unit_id: i64,
    pub name: String,
    pub slug: String,
    pub small_image: SmallImage,
    pub full_image: FullImage,
    pub description: Option<Description>,
    pub default_role: String,
    pub affiliation: String,
    pub rarity: String,
    pub element: String,
    pub path: String,
    pub stats: Stats,
    pub ascension_materials: AscensionMaterials,
    pub skills: Vec<Skill>,
    pub eidolon: Vec<Eidolon>,
    pub traces: Vec<Trace>,
    pub traces_small: Vec<TracesSmall>,
    pub review: Option<Review>,
    pub pros: Option<Pros>,
    pub cons: Option<Cons>,
    pub voice_actors: VoiceActors,
    pub ratings: Ratings,
    pub energy_ultimate: String,
    pub build_data: Option<Vec<BuildDaum>>,
    pub videos: Value,
    pub teams: Option<Vec<Team>>,
    pub character_builder_info: CharacterBuilderInfo,
    pub tier_category: String,
    pub release_date: Option<String>,
    pub is_released: bool,
    pub is_new: Value,
    pub is_updated: Value,
    pub hide_skills: bool,
    pub is_review_pending: bool,
    pub available_in_cbt3: Option<bool>,
    pub show_build: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallImage {
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
pub struct FullImage {
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
pub struct Description {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    #[serde(rename = "hp_base")]
    pub hp_base: f64,
    #[serde(rename = "def_base")]
    pub def_base: f64,
    #[serde(rename = "atk_base")]
    pub atk_base: f64,
    #[serde(rename = "speed_base")]
    pub speed_base: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AscensionMaterials {
    #[serde(rename = "mats_1")]
    pub mats_1: String,
    #[serde(rename = "mats_2")]
    pub mats_2: String,
    #[serde(rename = "mats_3")]
    pub mats_3: String,
    #[serde(rename = "mats_4")]
    pub mats_4: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub skill_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub enhanced_name: Value,
    pub tags: Option<String>,
    pub description_level1: DescriptionLevel1,
    #[serde(rename = "descriptionLevel1Enhanced")]
    pub description_level1enhanced: Value,
    pub has_enhanced_version: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionLevel1 {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eidolon {
    pub eidolon_id: String,
    pub name: String,
    #[serde(rename = "upgrade1Name")]
    pub upgrade1name: String,
    #[serde(rename = "upgrade1Desc")]
    pub upgrade1desc: Upgrade1Desc,
    #[serde(rename = "upgrade2Name")]
    pub upgrade2name: String,
    #[serde(rename = "upgrade2Desc")]
    pub upgrade2desc: Upgrade2Desc,
    #[serde(rename = "upgrade3Name")]
    pub upgrade3name: String,
    #[serde(rename = "upgrade3Desc")]
    pub upgrade3desc: Upgrade3Desc,
    #[serde(rename = "upgrade4Name")]
    pub upgrade4name: String,
    #[serde(rename = "upgrade4Desc")]
    pub upgrade4desc: Upgrade4Desc,
    #[serde(rename = "upgrade5Name")]
    pub upgrade5name: String,
    #[serde(rename = "upgrade5Desc")]
    pub upgrade5desc: Upgrade5Desc,
    #[serde(rename = "upgrade6Name")]
    pub upgrade6name: String,
    #[serde(rename = "upgrade6Desc")]
    pub upgrade6desc: Upgrade6Desc,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upgrade1Desc {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upgrade2Desc {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upgrade3Desc {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upgrade4Desc {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upgrade5Desc {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upgrade6Desc {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trace {
    pub req: String,
    pub desc: String,
    pub name: String,
    #[serde(rename = "sub_nodes")]
    pub sub_nodes: Vec<SubNode>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubNode {
    pub req: String,
    pub stat: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TracesSmall {
    pub req: String,
    pub stat: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pros {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cons {
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceActors {
    pub en: String,
    pub kr: String,
    pub jpn: String,
    pub cn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
    #[serde(rename = "story_early")]
    pub story_early: String,
    #[serde(rename = "story_late")]
    pub story_late: String,
    pub sim: String,
    pub bosses: String,
    pub farming: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildDaum {
    pub relics: Vec<Relic>,
    pub planars: Vec<Planar>,
    pub cones: Vec<Cone>,
    pub body: Vec<Body>,
    pub feet: Vec<Foot>,
    pub rope: Vec<Rope>,
    pub sphere: Vec<Sphere>,
    pub comments: String,
    pub substats: String,
    #[serde(rename = "skill_priority")]
    pub skill_priority: String,
    #[serde(rename = "traces_priority")]
    pub traces_priority: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relic {
    pub relic: String,
    #[serde(rename = "relic_2")]
    pub relic_2: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Planar {
    pub planar: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cone {
    pub cone: String,
    #[serde(rename = "super")]
    pub super_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub stat: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Foot {
    pub stat: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rope {
    pub stat: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sphere {
    pub stat: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub name: String,
    #[serde(rename = "member_1")]
    pub member_1: String,
    #[serde(rename = "member_2")]
    pub member_2: String,
    #[serde(rename = "member_3")]
    pub member_3: String,
    #[serde(rename = "member_4")]
    pub member_4: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterBuilderInfo {
    pub trace_stat1: TraceStat1,
    pub trace_stat2: TraceStat2,
    pub trace_stat3: TraceStat3,
    pub trace_stat_major1: TraceStatMajor1,
    pub trace_stat_major2: TraceStatMajor2,
    pub trace_stat_major3: TraceStatMajor3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceStat1 {
    pub stat: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceStat2 {
    pub stat: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceStat3 {
    pub stat: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceStatMajor1 {
    pub stat: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceStatMajor2 {
    pub stat: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceStatMajor3 {
    pub stat: String,
    pub value: i64,
}



impl PrydwenCompatible for PrydwenCharacter {}


pub async fn get_character_data(name: String) -> Option<Character> {
    let data_response = reqwest::get(format!("https://www.prydwen.gg/page-data/star-rail/characters/{}/page-data.json", name)).await.ok()?;
    let d = data_response.text().await.ok()?;
    let js = &mut serde_json::Deserializer::from_str(d.as_str());
    let data : Result<PrydwenResponse<PrydwenCharacter>, _> = serde_path_to_error::deserialize(js);
    match data {
        Ok(d) => {
            Some(d.result.data.current_unit.nodes.get(0).unwrap().clone())
        }
        Err(err) => {
            let path = err.path().to_string();
            println!("{}", path);
            None
        }
    }
}