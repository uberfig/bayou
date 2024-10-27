use super::super::{
    extensions::emoji::Emoji,
    serde_fns::{deserialize_time, serialize_time},
    structures::content_format::{ContentFormat, TextContentFormat},
};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NoteTyoe {
    Note,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Microblog,
    Forum,
    Blog,
    Image,
    Video,
    Audio,
    Messaging,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Groups {
    /// The note is visible to anyone.
    Public,
    /// The note is visible only to the author's followers.
    Followers,
    /// The note is visible only to users on the author's instance.
    Local,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupType {
    /// url to a [`super::group::Group`]
    Group(Url),
    Simple(Groups),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: NoteTyoe,
    pub uri: Url,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,
    /// Media attachments to the note. May be any format. Must be remote.
    pub attachments: Option<Vec<ContentFormat>>,
    /// URI of the User considered the author of the note.
    pub author: Url,
    /// Category of the note. Useful for clients to render
    /// notes differently depending on their intended purpose.
    pub category: Option<Category>,
    /// The content of the note. Must be text format
    /// (text/html, text/markdown, etc). Must not be remote.
    pub content: Option<ContentFormat>,
    /// Device used to post the note. Useful for functionality such as Twitter's "posted via" feature.
    pub device: Option<Device>,
    pub extensions: Option<NoteExtensions>,
    /// URI of a Group that the note is only visible in or one of the provided types
    ///
    /// If not provided, the note is only visible to the author and those mentioned in the note.
    pub group: Option<GroupType>,
    /// Whether the note contains "sensitive content".
    /// This can be used with [`Note::subject`] as a "content warning" feature.
    pub is_sensitive: Option<bool>,
    /// URIs of [Users](https://versia.pub/entities/user) that should be notified of the note.
    /// Similar to Twitter's @ mentions. The note may also
    /// contain mentions in the content, however only the
    /// mentions in this field should trigger notifications.
    pub mentions: Option<Vec<Url>>,
    /// used with [`Note::is_sensitive`] as a "content warning" feature.
    pub subject: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub version: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteExtensions {
    #[serde(rename = "pub.versia:custom_emojis")]
    pub pub_versia_custom_emojis: Option<PubVersiaCustomEmojis>,
    #[serde(rename = "pub.versia:polls")]
    pub pub_versia_polls: Option<PubVersiaPolls>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PubVersiaCustomEmojis {
    pub emojis: Vec<Emoji>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PubVersiaPolls {
    pub options: Vec<TextContentFormat>,
    /// Array of the number of votes for each option. The length of this array should match the length of the options array.
    pub votes: Vec<u64>,
    pub multiple_choice: bool,
    /// ISO 8601 timestamp of when the poll ends and no more votes can be cast.
    /// If not present, the poll does not expire.
    #[serde(flatten)]
    pub expires_at: Option<Expiry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expiry {
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub expires_at: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_poll() -> Result<(), String> {
        //taken from the versia protocol examples
        let poll = r#"
{
    "id": "01902e09-0f8b-72de-8ee3-9afc0cf5eae1",
    "type": "Note", 
    "uri": "https://versia.social/notes/01902e09-0f8b-72de-8ee3-9afc0cf5eae1",
    "created_at": "2024-06-19T01:07:44.139Z",
    "author": "https://versia.social/users/018eb863-753f-76ff-83d6-fd590de7740a",
    "category": "microblog",
    "content": {
        "text/plain": {
            "content": "What is your favourite color?"
        }
    },
    "extensions": { 
        "pub.versia:polls": {
            "options": [
                {
                    "text/plain": {
                        "content": "Red"
                    }
                },
                {
                    "text/plain": {
                        "content": "Blue"
                    }
                },
                {
                    "text/plain": {
                        "content": "Green"
                    }
                }
            ],
            "votes": [
                9,
                5,
                0
            ],
            "multiple_choice": false,
            "expires_at": "2021-01-04T00:00:00.000Z"
        }
    },
    "group": "public",
    "is_sensitive": false,
    "mentions": []
}
"#;
        let deserialized: Result<Note, serde_json::Error> = serde_json::from_str(poll);
        match deserialized {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("poll deserialize failed: {}", x)),
        }
    }
}
