use super::super::serde_fns::{default_false, default_true};
use serde::{Deserialize, Serialize};

use url::Url;

/// The ContentFormat structure is used to represent content with metadata.
/// It supports multiple content types for the same file, such as a PNG
/// image and a WebP image.
///
/// https://versia.pub/structures/content-format
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ContentFormat {
    Image(ImageContentFormat),
    Text(TextContentFormat),
    Audio(AudioContentFormat),
    Video(VideoContentFormat),
    // Unkown(HashMap<String, >)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageContentFormat {
    #[serde(rename = "image/png")]
    pub png: Option<ImageContent>,
    #[serde(rename = "image/jpg")]
    pub jpg: Option<ImageContent>,
    #[serde(rename = "image/heif")]
    pub heif: Option<ImageContent>,
    #[serde(rename = "image/webp")]
    pub webp: Option<ImageContent>,
    #[serde(rename = "image/avif")]
    pub avif: Option<ImageContent>,
    #[serde(rename = "image/gif")]
    pub gif: Option<ImageContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextContentFormat {
    #[serde(rename = "text/html")]
    pub html: Option<TextContent>,
    #[serde(rename = "text/markdown")]
    pub markdown: Option<TextContent>,
    #[serde(rename = "text/x.misskeymarkdown")]
    pub misskeymarkdown: Option<TextContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioContentFormat {
    #[serde(rename = "audio/mp3")]
    pub mp3: Option<AudioContent>,
    #[serde(rename = "audio/ogg")]
    pub ogg: Option<AudioContent>,
    #[serde(rename = "audio/wav")]
    pub wav: Option<AudioContent>,
    #[serde(rename = "audio/flac")]
    pub flac: Option<AudioContent>,
    #[serde(rename = "audio/opus")]
    pub opus: Option<AudioContent>,
    #[serde(rename = "audio/aac")]
    pub aac: Option<AudioContent>,
    #[serde(rename = "audio/m4a")]
    pub m4a: Option<AudioContent>,
    #[serde(rename = "audio/3gp")]
    pub audio_3gp: Option<AudioContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoContentFormat {
    #[serde(rename = "video/mp4")]
    pub mp4: Option<VideoContent>,
    #[serde(rename = "video/m4v")]
    pub m4v: Option<VideoContent>,
    #[serde(rename = "video/mov")]
    pub mov: Option<VideoContent>,
    #[serde(rename = "video/webm")]
    pub webm: Option<VideoContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextOption {
    Remote(Url),
    Simple(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextContent {
    pub content: TextOption,
    #[serde(default = "default_false")]
    pub remote: bool,
    pub description: Option<String>,
    pub size: Option<u64>,
    pub hash: Option<Hash>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageContent {
    pub content: Url,
    #[serde(default = "default_true")]
    pub remote: bool, //should always be true
    pub description: Option<String>,
    pub size: Option<u64>,
    pub hash: Option<Hash>,
    pub thumbhash: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoContent {
    pub content: Url,
    #[serde(default = "default_true")]
    pub remote: bool, //should always be false
    pub description: Option<String>,
    pub size: Option<u64>,
    pub hash: Option<Hash>,
    pub thumbhash: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub fps: Option<u64>,
    pub duration: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioContent {
    pub content: Url,
    #[serde(default = "default_true")]
    pub remote: bool, //should always be false
    pub description: Option<String>,
    pub size: Option<u64>,
    pub hash: Option<Hash>,
    pub thumbhash: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub duration: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hash {
    #[serde(flatten)]
    pub value: HashType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum HashType {
    #[serde(rename = "sha256")]
    Sha256(String),
    #[serde(rename = "sha512")]
    Sha512(String),
    #[serde(rename = "sha3-256")]
    Sha3_256(String),
    #[serde(rename = "sha3-512")]
    Sha3_512(String),
    #[serde(rename = "blake2b-256")]
    Blake2b_256(String),
    #[serde(rename = "blake2b-512")]
    Blake2b_512(String),
    #[serde(rename = "blake3-256")]
    Blake3_256(String),
    #[serde(rename = "blake3-512")]
    Blake3_512(String),
    #[serde(rename = "md5")]
    Md5(String),
    #[serde(rename = "sha1")]
    Sha1(String),
    #[serde(rename = "sha224")]
    Sha224(String),
    #[serde(rename = "sha384")]
    Sha384(String),
    #[serde(rename = "sha3-224")]
    Sha3_224(String),
    #[serde(rename = "sha3-384")]
    Sha3_384(String),
    #[serde(rename = "blake2s-256")]
    Blake2s_256(String),
    #[serde(rename = "blake2s-512")]
    Blake2s_512(String),
    #[serde(rename = "blake3-224")]
    Blake3_224(String),
    #[serde(rename = "blake3-384")]
    Blake3_384(String),
}
