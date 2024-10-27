use serde::Deserializer;
use serde::{de::Error as DeError, Deserialize, Serialize};
use url::Url;

use super::super::extensions::emoji::Emoji;
use super::super::serde_fns::{default_false, default_true, deserialize_time, serialize_time};
use super::super::structures::content_format::{ImageContentFormat, TextContentFormat};
#[cfg(feature = "crypto")]
use super::public_key::VersiaPublicKey;

/// Users are identified by their id property, which is unique within the instance.
///
/// https://versia.pub/entities/user
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: UserType,
    pub uri: Url,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,

    /// The user's avatar. Must be an image format (image/*).
    pub avatar: Option<ImageContentFormat>,
    /// Short description of the user. Must be text format (text/*).
    pub bio: Option<TextContentFormat>,
    /// Display name, as shown to other users.
    /// May contain emojis and any Unicode character.
    pub display_name: Option<String>,
    /// Custom key/value pairs. For example, metadata like socials or pronouns.
    /// Must be text format (text/*).
    pub fields: Option<Vec<Field>>,
    /// Alpha-numeric username. Must be unique within the instance.
    /// **Must** be treated as changeable by the user.
    ///
    /// Can only contain the following characters:
    /// - `a-z` (lowercase),
    /// - `0-9`, `_` and `-` using the regex `[^\da-z_\-]` to check for invalid
    /// - Should be limited to reasonable lengths.
    #[serde(deserialize_with = "deserialize_username")]
    pub username: String,
    /// A header image for the user's profile.
    /// Also known as a cover photo or a banner.
    /// Must be an image format (image/*).
    pub header: Option<ImageContentFormat>,
    #[cfg(feature = "crypto")]
    /// see [`PublicKey`]
    pub public_key: VersiaPublicKey,
    /// If true, the user must approve any new followers manually.
    /// If false, followers are automatically approved.
    /// This does not affect federation, and is meant to be used for
    /// clients to display correct UI. Defaults to false.
    #[serde(default = "default_false")]
    pub manually_approves_followers: bool,
    /// User consent to be indexed by search engines. If false, the
    /// user's profile should not be indexed. Defaults to true.
    #[serde(default = "default_true")]
    pub indexable: bool,
    /// The user's federation inbox. Refer to the federation documentation.
    /// Some instances may also have a shared inbox.
    /// Refer to [Instance Metadata](https://versia.pub/entities/instance-metadata)
    /// for more information.
    pub inbox: Url,
    /// Collections related to the user.
    /// Must contain at least `outbox`, `followers`, `following`, and `featured`.
    pub collections: UserCollections,
    pub extensions: Option<Extensions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserType {
    User,
}

fn deserialize_username<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let input = <&str>::deserialize(deserializer)?;
    if input.is_empty() {
        return Err(serde::de::Error::custom("username is empty"));
    }
    for char in input.chars() {
        if !matches!(char, 'a'..='z'| '0'..='9' | '_' | '-') {
            return Err(D::Error::custom("username contains invalid characters"));
        }
    }
    Ok(input.to_owned())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserCollections {
    pub outbox: Url,
    pub followers: Url,
    pub following: Url,
    pub featured: Url,

    #[serde(rename = "pub.versia:likes/Dislikes")]
    pub pub_versia_likes_dislikes: Option<Url>,
    #[serde(rename = "pub.versia:likes/Likes")]
    pub pub_versia_likes_likes: Option<Url>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub key: TextContentFormat,
    pub value: TextContentFormat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extensions {
    #[serde(rename = "pub.versia:custom_emojis")]
    pub pub_versia_custom_emojis: Option<PubVersiaCustomEmojis>,
    #[serde(rename = "pub.versia:migration")]
    pub pub_versia_migration: Option<PubVersiaMigration>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PubVersiaCustomEmojis {
    pub emojis: Vec<Emoji>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrevOrNew {
    /// If this user has migrated from another instance, this property MUST be set to the URI of the user on the previous instance
    Previous(Url),
    /// If this user has migrated to another instance, this property MUST be set to the URI of the user on the new instance.
    New(Url),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PubVersiaMigration {
    #[serde(flatten)]
    pub target: PrevOrNew,
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "crypto")]
    use super::super::super::entities::public_key::{AlgorithmsPublicKey, Ed25519Public};
    #[cfg(feature = "crypto")]
    use ed25519_dalek::SigningKey;
    #[cfg(feature = "crypto")]
    use rand::rngs::OsRng;

    use super::*;

    #[cfg(feature = "crypto")]
    fn generate_verifying_key() -> ed25519_dalek::VerifyingKey {
        let mut csprng = OsRng;
        let key = SigningKey::generate(&mut csprng);
        key.verifying_key()
    }

    #[test]
    fn test_serialize() -> Result<(), String> {
        let user = User {
            id: "018ec082-0ae1-761c-b2c5-22275a611771".to_string(),
            type_field: UserType::User,
            uri: Url::parse("https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771")
                .unwrap(),
            created_at: 1726590522000,
            avatar: None,
            bio: None,
            display_name: None,
            fields: None,
            username: "ivy".to_string(),
            header: None,
            #[cfg(feature = "crypto")]
            public_key: VersiaPublicKey {
                actor: None,
                key: AlgorithmsPublicKey::Ed25519(Ed25519Public {
                    key: generate_verifying_key(),
                }),
            },
            manually_approves_followers: true,
            indexable: true,
            inbox: Url::parse(
                "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/inbox",
            )
            .unwrap(),
            collections: UserCollections {
                outbox: Url::parse(
                    "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/inbox",
                )
                .unwrap(),
                followers: Url::parse(
                    "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/inbox",
                )
                .unwrap(),
                following: Url::parse(
                    "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/inbox",
                )
                .unwrap(),
                featured: Url::parse(
                    "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/inbox",
                )
                .unwrap(),
                pub_versia_likes_dislikes: None,
                pub_versia_likes_likes: None,
            },
            extensions: None,
        };

        let serialized = serde_json::to_string(&user);
        let serialized = match serialized {
            Ok(ok) => ok,
            Err(err) => return Err(format!("serialize failed {}", err)),
        };

        let deserialized: Result<User, _> = serde_json::from_str(&serialized);
        let _deserialized = match deserialized {
            Ok(ok) => ok,
            Err(err) => {
                return Err(format!(
                    "failed to deserialize the serialized value {}",
                    err
                ))
            }
        };

        Ok(())
    }

    #[test]
    #[cfg(feature = "crypto")]
    fn test_deserialize() -> Result<(), String> {
        //taken from the versia protocol examples
        let key = Ed25519Public {
            key: generate_verifying_key(),
        };
        let key = serde_json::to_string(&key);
        let key = match key {
            Ok(x) => x,
            Err(x) => return Err(format!("failed to deserialize key {}", x)),
        };
        let versia_user = format!(
            r#"
{{
    "id": "018ec082-0ae1-761c-b2c5-22275a611771",
    "type": "User",
    "uri": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771",
    "created_at": "2024-04-09T01:38:51.743Z",
    "avatar": {{ 
        "image/png": {{
            "content": "https://avatars.githubusercontent.com/u/30842467?v=4"
        }}
    }},
    "bio": {{
        "text/html": {{
            "content": "<p>🌸🌸🌸</p>"
        }},
        "text/plain": {{
            "content": "🌸🌸🌸"
        }}
    }},
    "collections": {{
        "featured": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/featured",
        "followers": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/followers",
        "following": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/following",
        "pub.versia:likes/Dislikes": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/dislikes",
        "pub.versia:likes/Likes": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/likes",
        "outbox": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/outbox"
    }},
    "display_name": "April The Pink (limited Sand Edition)",
    "extensions": {{
        "pub.versia:custom_emojis": {{
            "emojis": []
        }}
    }},
    "fields": [
        {{
            "key": {{
                "text/html": {{
                    "content": "<p>Pronouns</p>"
                }}
            }},
            "value": {{
                "text/html": {{
                    "content": "<p>It/its</p>"
                }}
            }}
        }}
    ],
    "header": null,
    "inbox": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771/inbox",
    "indexable": false,
    "manually_approves_followers": false,
    "public_key": {{
        "actor": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771",
        "algorithm": "ed25519",
        "key": {}
    }},
    "username": "aprl"
}}
        "#,
            key
        );
        // println!("{}", &versia_user);
        let deserialized: Result<User, serde_json::Error> = serde_json::from_str(&versia_user);
        match deserialized {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("user deserialize failed: {}", x)),
        }
    }
}
