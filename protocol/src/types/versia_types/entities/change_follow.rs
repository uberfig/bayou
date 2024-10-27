use super::super::serde_fns::{deserialize_time, serialize_time};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeFollowType {
    /// https://versia.pub/entities/follow
    Follow,
    /// https://versia.pub/entities/unfollow
    Unfollow,
}

/// actions taken by the follower
///
/// the author requests to follow or unfollow the followee
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeFollowing {
    #[serde(rename = "type")]
    pub type_field: ChangeFollowType,
    pub id: String,
    pub author: Url,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,
    pub followee: Url,
}
