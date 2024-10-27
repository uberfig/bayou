use serde::{Deserialize, Serialize};
use url::Url;

use super::super::versia_types::serde_fns::{deserialize_time, serialize_time};

use super::postable::ApPostable;
use super::{
    actors::Actor,
    collections::ExtendsCollection,
    core_types::OptionalArray,
    link::{LinkSimpleOrExpanded, RangeLinkItem},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MediaType {
    #[serde(rename = "text/html")]
    Html,
    #[serde(rename = "text/markdown")]
    Markdown,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum ObjectType {
    #[default]
    Object,
    Relationship, //adds properties: subject | object | relationship

    Document,
    Audio, //document type
    Image, //document type
    Video, //document type
    Page,  //document type

    Event,
    Place, //not used, adds  accuracy | altitude | latitude | longitude | radius | units

    Profile, // adds describes
    /// A Tombstone represents a content object that has
    /// been deleted. It can be used in Collections to
    /// signify that there used to be an object at this
    /// position, but it has been deleted.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tombstone
    Tombstone, // adds formerType | deleted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(rename = "type")]
    pub type_field: ObjectType,
    pub id: Url,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    pub attributed_to: RangeLinkItem<Actor>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies the entity (e.g. an application) that generated the object
    pub generator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<Url>,

    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub published: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<OptionalArray<RangeLinkItem<Actor>>>,

    //TODO
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub attachment: Option<String>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub start_time: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<OptionalArray<LinkSimpleOrExpanded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<OptionalArray<LinkSimpleOrExpanded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies an Object that is part of the private primary audience of this Object.
    pub bto: Option<OptionalArray<LinkSimpleOrExpanded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies an Object that is part of the public secondary audience of this Object.
    pub cc: Option<OptionalArray<LinkSimpleOrExpanded>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifies one or more Objects that are part of the private secondary audience of this Object.
    pub bcc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<ExtendsCollection<ApPostable>>,
}
