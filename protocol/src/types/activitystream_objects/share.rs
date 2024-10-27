use super::super::versia_types::serde_fns::{deserialize_time, serialize_time};
use serde::{Deserialize, Serialize};
use url::Url;

use super::context::Context;
use super::core_types::OptionalArray;
use super::link::LinkSimpleOrExpanded;
use super::{link::RangeLinkItem, note::Note, question::Question};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ShareType {
    Announce,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Shareable {
    Note(Note),
    Question(Question),
}

/// Indicates that the actor is calling the target's attention the object.
/// The origin typically has no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-announce
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    #[serde(rename = "type")]
    pub type_field: ShareType,
    pub id: Url,
    pub actor: Url,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub published: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<OptionalArray<LinkSimpleOrExpanded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Should typically have the object's creator and the
    /// booster's follower's collection
    pub cc: Option<OptionalArray<LinkSimpleOrExpanded>>,

    pub object: RangeLinkItem<Shareable>,
}

impl Share {
    pub fn get_context() -> Context {
        Context::Single("https://www.w3.org/ns/activitystreams".to_owned())
    }
}
