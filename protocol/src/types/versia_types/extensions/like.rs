use super::super::serde_fns::{deserialize_time, serialize_time};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LikeType {
    #[serde(rename = "pub.versia:likes/Like")]
    Like,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Like {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: LikeType,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,
    pub uri: Url,

    pub author: Url,
    pub liked: Url,
}
