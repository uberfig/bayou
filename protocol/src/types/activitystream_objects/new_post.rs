use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    core_types::OptionalArray,
    link::LinkSimpleOrExpanded,
    note::{MediaType, NoteType},
    question::{ChoiceType, QuestionType},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum NewPost {
    NewNote(NewNote),
    NewQuestion(NewQuestion),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNote {
    #[serde(rename = "type")]
    pub type_field: NoteType,
    pub id: Option<Url>,
    pub attributed_to: Url,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_for_later: Option<OptionalArray<LinkSimpleOrExpanded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<OptionalArray<LinkSimpleOrExpanded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<OptionalArray<LinkSimpleOrExpanded>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewQuestion {
    pub id: Option<Url>,
    pub actor: Url,
    #[serde(rename = "type")]
    pub type_field: QuestionType,
    #[serde(flatten)]
    pub options: ChoiceType,

    /// indicates that a poll can only be voted on by local users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<String>, //TODO

    #[serde(skip_serializing_if = "Option::is_none")]
    pub versia_url: Option<Url>,
}
