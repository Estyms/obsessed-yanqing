use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrydwenResponse<T: PrydwenCompatible> {
    pub component_chunk_name: String,
    pub path: String,
    pub result: Result<T>,
    pub static_query_hashes: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result<T: PrydwenCompatible> {
    pub data: T,
    pub page_context: PageContext
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageContext {
    contentful_id: Option<String>
}

pub trait PrydwenCompatible {
}