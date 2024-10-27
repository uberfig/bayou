use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LinkType {
    Link,
    /// A specialized Link that represents an @mention.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-mention
    Mention,
    /// A specialized Link that represents a topic such as #topic
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#h-microsyntaxes
    Hashtag,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    #[serde(rename = "type")]
    pub type_field: LinkType,

    pub href: Url,
    pub name: Option<String>,
    // pub hreflang: Option<String>,
    // pub media_type: Option<String>,

    // pub height: Option<u32>,
    // pub width: Option<u32>,
    // pub preview: Option<String>, //TODO
    // pub rel: Option<String>,     //TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LinkSimpleOrExpanded {
    Simple(Url),
    Expanded(Link),
}

impl LinkSimpleOrExpanded {
    pub fn get_id(&self) -> &Url {
        match self {
            LinkSimpleOrExpanded::Simple(x) => x,
            LinkSimpleOrExpanded::Expanded(x) => &x.href,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// represents a field that could be an item of type T or a link
pub enum RangeLinkItem<T> {
    Item(T),
    Link(LinkSimpleOrExpanded),
}

#[cfg(test)]
mod tests {

    use super::super::{core_types::*, link::*};

    #[test]
    fn test_tags() -> Result<(), String> {
        let tags = r##"

    [
		{
			"type": "Mention",
			"href": "https://mastodon.social/users/mellifluousbox",
			"name": "@mellifluousbox"
		},
		{
			"type": "Mention",
			"href": "https://mastodon.social/users/Gargron",
			"name": "@Gargron"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/finance",
			"name": "#finance"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/ops",
			"name": "#ops"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/remote",
			"name": "#remote"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/german",
			"name": "#german"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/accounting",
			"name": "#accounting"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/bookkeeping",
			"name": "#bookkeeping"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/legal",
			"name": "#legal"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/excel",
			"name": "#excel"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/internship",
			"name": "#internship"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/FediHire",
			"name": "#FediHire"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/getfedihired",
			"name": "#getfedihired"
		},
		{
			"type": "Hashtag",
			"href": "https://mastodon.social/tags/hiring",
			"name": "#hiring"
		}
	]

        "##;

        let deserialized: Result<OptionalArray<Link>, serde_json::Error> =
            serde_json::from_str(tags);
        let _deserialized = match deserialized {
            Ok(x) => x,
            Err(x) => return Err(format!("tag array failed to deserialize: {}", x)),
        };

        Ok(())
    }
}
