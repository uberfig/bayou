use super::super::{
    serde_fns::{deserialize_time, serialize_time},
    structures::content_format::ContentFormat,
};
use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(feature = "crypto")]
use super::public_key::VersiaPublicKey;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InstanceMetadataType {
    InstanceMetadata,
}

/// Contains metadata about a Versia instance, such as capabilities and endpoints.
///
/// On all entities that have an author field, the author can be null to
/// represent the instance itself as the author (like ActivityPub's Server
/// Actors). In this case, the instance's public key should be used to
/// verify the entity.
///
/// https://versia.pub/entities/instance-metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceMetadata {
    #[serde(rename = "type")]
    pub type_field: InstanceMetadataType,
    /// Friendly name of the instance, for humans.
    pub name: String,
    /// Information about the software running the instance.
    pub software: Software,
    /// Information about the compatibility of the instance.
    pub compatibility: Compatibility,
    /// Long description of the instance, for humans.
    /// Should be around 100-200 words.
    pub description: Option<String>,
    /// Hostname of the instance. Includes the port if it is not the default
    /// (i.e. 443 for HTTPS).
    pub host: String,
    /// URI to the instance's shared inbox, if supported.
    pub shared_inbox: Option<Url>,
    /// URI to [Collection](https://versia.pub/structures/collection) of instance moderators.
    pub moderators: Option<Url>,
    /// URI to [Collection](https://versia.pub/structures/collection) of instance administrators.
    pub admins: Option<Url>,
    /// Logo of the instance. Must be an image format (image/*).
    pub logo: Option<ContentFormat>,
    #[cfg(feature = "crypto")]
    /// Public key of the instance.
    pub public_key: VersiaPublicKey,
    /// Banner of the instance. Must be an image format (image/*).
    pub banner: Option<ContentFormat>,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,
    // TODO
    // pub extensions: Extensions,
}
const VERSION: &str = env!("CARGO_PKG_VERSION");
const SOFTWARE_NAME: &str = env!("CARGO_PKG_NAME");

impl InstanceMetadata {
    /// generates a new instance metadata for the local server.
    /// when generating one for another instance, for example
    /// getting it from the database, all fields should be
    /// filled manually
    pub fn new(
        name: String,
        description: Option<String>,
        host: String,
        logo: Option<ContentFormat>,
        #[cfg(feature = "crypto")] public_key: VersiaPublicKey,
        banner: Option<ContentFormat>,
        created_at: i64,
    ) -> InstanceMetadata {
        InstanceMetadata {
            type_field: InstanceMetadataType::InstanceMetadata,
            name,
            software: Software {
                name: SOFTWARE_NAME.to_string(),
                version: VERSION.to_string(),
            },
            compatibility: Compatibility {
                versions: vec!["0.4".to_string()],
                extensions: vec![],
            },
            description,
            host: host.clone(),
            shared_inbox: Some(Url::parse(&format!("https://{}/inbox", &host)).unwrap()),
            moderators: Some(Url::parse(&format!("https://{}/moderators", &host)).unwrap()),
            admins: Some(Url::parse(&format!("https://{}/admins", &host)).unwrap()),
            logo,
            #[cfg(feature = "crypto")]
            public_key,
            banner,
            created_at,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Information about the software running the instance.
pub struct Software {
    /// Name of the software.
    pub name: String,
    /// Version of the software. Should use [SemVer](https://semver.org/)
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Information about the compatibility of the instance.
pub struct Compatibility {
    pub versions: Vec<String>,
    pub extensions: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extensions {
    #[serde(rename = "example.extension:monthly_active_users")]
    pub example_extension_monthly_active_users: i64,
}
