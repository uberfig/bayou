use super::super::serde_fns::{deserialize_time, serialize_time};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShareType {
    #[serde(rename = "pub.versia:share/Share")]
    Share,
}

/// The Share Extension lets users share notes they like with others.
/// This is the same as Twitter's "retweet" and Mastodon's "boost".
///
/// https://versia.pub/extensions/share
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Share {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: ShareType,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,
    pub author: Url,
    pub uri: Url,
    pub shared: Url,
}
