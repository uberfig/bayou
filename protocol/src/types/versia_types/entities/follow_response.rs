use super::super::serde_fns::{deserialize_time, serialize_time};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FollowResponseType {
    /// the author accepts follower's request
    ///
    /// https://versia.pub/entities/follow-accept
    FollowAccept,
    /// FollowReject can also be used after a follow relationship has
    /// been established to remove a follower or to reject a new request
    ///
    /// https://versia.pub/entities/follow-reject
    FollowReject,
}

/// actions taken by the followee
///
/// the author accepts or rejects the follower's request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowResponse {
    #[serde(rename = "type")]
    pub type_field: FollowResponseType,
    pub id: String,
    pub author: Url,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,
    pub follower: Url,
}
