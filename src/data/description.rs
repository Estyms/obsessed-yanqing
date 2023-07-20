use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use crate::data::character::{Cons, Description, Pros, Review};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    pub content: Vec<Content>,
    pub node_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub data: Data,
    pub content: Vec<Content2>,
    pub node_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content2 {
    pub data: Data,
    pub marks: Vec<Value>,
    pub value: String,
    pub node_type: String,
}

pub trait RawText {
    fn get_raw(&self) -> &str;
}

impl RawText for &Description {
    fn get_raw(&self) -> &str {
        self.raw.as_str()
    }
}

impl RawText for &Review {
    fn get_raw(&self) -> &str {
        self.raw.as_str()
    }
}

impl RawText for &Cons {
    fn get_raw(&self) -> &str {
        self.raw.as_str()
    }
}

impl RawText for &Pros {
    fn get_raw(&self) -> &str {
        self.raw.as_str()
    }
}

pub fn get_all_texts<T:RawText>(desc: &T) -> Option<Vec<String>> {
    let js = &mut serde_json::Deserializer::from_str(desc.get_raw());
    let data : Result<Root, _> = serde_path_to_error::deserialize(js);
    match data {
        Ok(d) => {
            Some(d.content.into_iter().flat_map(|x| x.content.into_iter().map(|y| y.value)).collect())
        }
        Err(err) => {
            let path = err.path().to_string();
            println!("{}", path);
            None
        }
    }
}