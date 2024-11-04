use serde::{Deserialize, Serialize};
use url::Url;

use super::{context::Context, note::Note};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QuestionType {
    Question,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Represents a question being asked.
/// Question objects are an extension of [`IntransitiveActivity`]. That is,
/// the Question object is an Activity, but the direct object is the question
/// itself and therefore it would not contain an object property.
///
/// Either of the anyOf and oneOf properties MAY be used to express possible answers,
/// but a Question object MUST NOT have both properties.
///
/// Commonly used for polls
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-question
pub struct Question {
    pub id: Url,
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

impl Question {
    pub fn get_context() -> Context {
        Note::get_context()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ChoiceType {
    AnyOf(Vec<QuestionOption>),
    OneOf(Vec<QuestionOption>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QuestionOptionType {
    Note,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuestionOption {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: QuestionOptionType,
}
