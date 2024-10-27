use super::super::structures::content_format::ImageContentFormat;
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Emoji {
    #[serde(deserialize_with = "deserialize_name")]
    #[serde(serialize_with = "serialize_name")]
    pub name: EmojiName,
    pub content: ImageContentFormat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmojiName {
    pub shortcode: String,
    pub identifier: char,
}

fn deserialize_name<'de, D>(deserializer: D) -> Result<EmojiName, D::Error>
where
    D: Deserializer<'de>,
{
    let input = <&str>::deserialize(deserializer)?;

    let mut chars = input.chars();
    let Some(first) = chars.next() else {
        return Err(D::Error::custom("emoji first identifier missing"));
    };
    let Some(last) = chars.next_back() else {
        return Err(D::Error::custom("emoji last identifier missing"));
    };
    let shortcode = chars.as_str();

    if first != last {
        return Err(D::Error::custom("emoji first identifiers don't match"));
    }

    if matches!(first, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | ' ') {
        return Err(D::Error::custom(
            "emoji first identifiers cannot match as name character",
        ));
    }
    if input.is_empty() {
        return Err(D::Error::custom("emoji name is empty"));
    }
    for char in shortcode.chars() {
        if !matches!(char, 'a'..='z'| 'A'..='Z' | '0'..='9' | '_' | '-') {
            return Err(D::Error::custom("shortcode contains invalid characters"));
        }
    }

    Ok(EmojiName {
        shortcode: shortcode.to_owned(),
        identifier: first,
    })
}

pub fn serialize_name<S>(x: &EmojiName, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{}{}{}", x.identifier, x.shortcode, x.identifier))
}
