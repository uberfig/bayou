use serde::{Deserialize, Serialize};
use url::Url;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    /// indicates that a poll can only be voted on by local users
    pub local_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<String>, //TODO
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