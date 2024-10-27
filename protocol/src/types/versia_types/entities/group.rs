use serde::{Deserialize, Serialize};
use url::Url;

use super::super::structures::content_format::TextContentFormat;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GroupType {
    Group,
}

/// Groups are a way to organize users and notes into communities.
/// They can be used for any purpose, such as forums, blogs,
/// image galleries, video sharing, audio sharing, and messaging.
/// They are similar to Discord's channels or Matrix's rooms.
///
/// Refer to [`super::notes::Note`]'s [`super::notes::Note::group`]
/// property for how notes can be associated with groups.
///
/// https://versia.pub/entities/group
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "type")]
    pub type_field: GroupType,
    pub id: String,
    pub uri: Url,
    pub name: Option<TextContentFormat>,
    pub description: Option<TextContentFormat>,
    pub members: Url,
    pub notes: Option<Url>,
}
