//! emoji and emoji packs will be located at
//! defined spaces with the prefix /.well-known/bayou
//! as per the swamptalk protocol
//!
//! - packs will be located at `/.well-known/bayou/emoji/{pack id}`
//! - custom emoji will then be located at
//! `/.well-known/bayou/emoji/{pack id}/{shortcode}`
//!
//! custom emoji should only link to media of the supported emoji
//! as listed in [`EmojiFormat`] to guarentee support
//!
//! shortcodes must be lowercase and they must not contain
//! whitespace or colons. colons will be used as delimiters in
//! inline text such that a simple replace all `:shortcode:`
//! with the emoji listed in the items used custom emoji should
//! work.
//!
//! ***Important*** shortcodes in [`EmojiFederation`] may not
//! contain hyphens but [`InlineEmojiFederation`] may so they
//! may use `:shortcode-1:` and `:shortcode-2:` or
//! `:shortcode-uuid:` or something of the sort to help with
//! differentiating overlap
//!
//!
//! Messages that contain custom emoji will have a list of
//! [`InlineEmojiFederation`] which allow for them to change
//! the shortcodes as necessary to prevent overlap

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::media::Media;

/// these will be the only supported formats for custom emoji
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EmojiFormat {
    #[serde(rename = "image/png")]
    PNG,
    #[serde(rename = "image/jpg")]
    JPG,
    #[serde(rename = "image/gif")]
    GIF,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmojiPackFederation {
    pub external_pack_id: Uuid,
    /// shortcode of the highlight emoji
    pub highlight_emoji: String,
    pub pack_name: String,
    pub description: Option<String>,
    pub author_url: Option<Url>,
    pub emoji: Vec<EmojiFederation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmojiFederation {
    pub external_pack_id: Uuid,
    pub domain: String,
    /// must be unique within the emoji pack, no dupes!
    /// no whitespace, no colons, all lowercase.
    /// Implimentors may chose to reject emoji with
    /// capitals or convert to lowercase
    pub shortcode: String,
    pub icon: Media<EmojiFormat>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InlineEmojiFederation {
    pub emoji: EmojiFederation,
    /// follows the same conventions of shortcodes
    /// can be appended to or changed however clients
    /// please to stop shortcode overlap
    pub effective_shortcode: Option<String>,
}
