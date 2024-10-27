use serde::{Deserialize, Serialize};

use super::{entities::notes::Note, extensions::share::Share};

/// entities that can be "posted" by a user, ie what can be
/// in a users timeline. includes notes, shares, and polls
///
/// note that polls are contained within extensions to note
/// and not their own type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum VersiaPostable {
    Note(Box<Note>),
    Share(Share),
}

impl VersiaPostable {
    pub fn get_author(&self) -> &url::Url {
        match self {
            VersiaPostable::Note(note) => &note.author,
            VersiaPostable::Share(share) => &share.author,
        }
    }
    pub fn get_id(&self) -> &str {
        match self {
            VersiaPostable::Note(note) => &note.id,
            VersiaPostable::Share(share) => &share.id,
        }
    }
}
