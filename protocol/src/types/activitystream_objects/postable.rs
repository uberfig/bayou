#[cfg(feature = "protocol")]
use super::inboxable::InboxableVerifyErr;
use serde::{Deserialize, Serialize};
use url::Url;

use super::{context::ContextWrap, note::Note, question::Question, share::Share};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ApPostable {
    Question(Question),
    Note(Note),
    Share(Share),
}

impl ApPostable {
    pub fn id(&self) -> &Url {
        match self {
            ApPostable::Question(question) => &question.id,
            ApPostable::Note(note) => &note.id,
            ApPostable::Share(share) => &share.id,
        }
    }
    pub fn actor(&self) -> &Url {
        match self {
            ApPostable::Question(question) => &question.actor,
            ApPostable::Note(note) => &note.attributed_to,
            ApPostable::Share(share) => &share.actor,
        }
    }
    #[cfg(feature = "protocol")]
    pub fn verify(self, origin_domain: &str) -> Result<Self, InboxableVerifyErr> {
        if self.id().domain().ne(&Some(origin_domain))
            || self.actor().domain().ne(&Some(origin_domain))
        {
            return Err(InboxableVerifyErr::ForgedAttribution);
        }
        Ok(self)
    }
    pub fn wrap_context(self) -> ContextWrap<Self> {
        match self {
            ApPostable::Question(question) => ContextWrap {
                context: Question::get_context(),
                item: ApPostable::Question(question),
            },
            ApPostable::Note(note) => ContextWrap {
                context: Note::get_context(),
                item: ApPostable::Note(note),
            },
            ApPostable::Share(share) => ContextWrap {
                context: Share::get_context(),
                item: ApPostable::Share(share),
            },
        }
    }
}
