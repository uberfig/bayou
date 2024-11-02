use std::fmt::Display;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebfingerResult {
    pub subject: String,
    pub aliases: Option<Vec<String>>,
    pub links: Vec<WebfingerLink>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebfingerLink {
    pub rel: RelWrap,
    #[serde(rename = "type")]
    pub type_field: TypeWrap,
    pub href: Url,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum TypeWrap {
    Defined(WebfingerLinkTypes),
    Unkown(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum WebfingerLinkTypes {
    #[serde(rename = "application/activity+json")]
    Activitypub,
    #[serde(rename = "text/html")]
    Webpage,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum RelWrap {
    Defined(RelTypes),
    Unkown(String),
}

impl Display for RelWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelWrap::Defined(rel_types) => {
                write!(f, "{}", serde_json::to_string(rel_types).unwrap())
            }
            RelWrap::Unkown(unkown) => write!(f, "{}", unkown),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum RelTypes {
    #[serde(rename = "self")]
    RelSelf,
    #[serde(rename = "http://webfinger.net/rel/profile-page")]
    ProfilePage,
}
