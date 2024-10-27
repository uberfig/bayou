use super::{context::ContextWrap, public_key::ApPublicKey};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActorType {
    // Actor,
    /// Describes a software application.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-application
    Application,
    /// Represents a formal or informal collective of Actors.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-group
    Group,
    /// Represents an organization.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-organization
    Organization,
    /// Represents an individual person. The most
    /// common type of actor on the fedi
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-person
    Person,
    /// Represents a service of any kind.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-service
    Service,
}

//-------------------types--------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Actor types are [`Object`] types that are capable of performing activities
///
/// core types:
/// - [`ActorType::Application`]
/// - [`ActorType::Group`]
/// - [`ActorType::Organization`]
/// - [`ActorType::Person`]
/// - [`ActorType::Service`]
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#actor-types
pub struct Actor {
    #[serde(rename = "type")]
    pub type_field: ActorType,
    pub id: Url,
    pub preferred_username: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    pub public_key: ApPublicKey,

    pub inbox: Url,
    pub outbox: Url,
    pub followers: Url,
    pub following: Url,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub versia_url: Option<Url>,
}

impl Actor {
    pub fn wrap_context(self) -> ContextWrap<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() -> Result<(), String> {
        //taken from https://mastodon.social/users/Mastodon
        let mastodon_account = r#"
{
	"@context": [
		"https://www.w3.org/ns/activitystreams",
		"https://w3id.org/security/v1",
		{
			"manuallyApprovesFollowers": "as:manuallyApprovesFollowers",
			"toot": "http://joinmastodon.org/ns#",
			"featured": {
				"@id": "toot:featured",
				"@type": "@id"
			},
			"featuredTags": {
				"@id": "toot:featuredTags",
				"@type": "@id"
			},
			"alsoKnownAs": {
				"@id": "as:alsoKnownAs",
				"@type": "@id"
			},
			"movedTo": {
				"@id": "as:movedTo",
				"@type": "@id"
			},
			"schema": "http://schema.org#",
			"PropertyValue": "schema:PropertyValue",
			"value": "schema:value",
			"discoverable": "toot:discoverable",
			"Device": "toot:Device",
			"Ed25519Signature": "toot:Ed25519Signature",
			"Ed25519Key": "toot:Ed25519Key",
			"Curve25519Key": "toot:Curve25519Key",
			"EncryptedMessage": "toot:EncryptedMessage",
			"publicKeyBase64": "toot:publicKeyBase64",
			"deviceId": "toot:deviceId",
			"claim": {
				"@type": "@id",
				"@id": "toot:claim"
			},
			"fingerprintKey": {
				"@type": "@id",
				"@id": "toot:fingerprintKey"
			},
			"identityKey": {
				"@type": "@id",
				"@id": "toot:identityKey"
			},
			"devices": {
				"@type": "@id",
				"@id": "toot:devices"
			},
			"messageFranking": "toot:messageFranking",
			"messageType": "toot:messageType",
			"cipherText": "toot:cipherText",
			"suspended": "toot:suspended",
			"memorial": "toot:memorial",
			"indexable": "toot:indexable",
			"focalPoint": {
				"@container": "@list",
				"@id": "toot:focalPoint"
			}
		}
	],
	"id": "https://mastodon.social/users/Mastodon",
	"type": "Person",
	"following": "https://mastodon.social/users/Mastodon/following",
	"followers": "https://mastodon.social/users/Mastodon/followers",
	"inbox": "https://mastodon.social/users/Mastodon/inbox",
	"outbox": "https://mastodon.social/users/Mastodon/outbox",
	"featured": "https://mastodon.social/users/Mastodon/collections/featured",
	"featuredTags": "https://mastodon.social/users/Mastodon/collections/tags",
	"preferredUsername": "Mastodon",
	"name": "Mastodon",
	"summary": "<p>Free, open-source decentralized social media platform.</p>",
	"url": "https://mastodon.social/@Mastodon",
	"manuallyApprovesFollowers": false,
	"discoverable": true,
	"indexable": false,
	"published": "2016-11-23T00:00:00Z",
	"memorial": false,
	"devices": "https://mastodon.social/users/Mastodon/collections/devices",
	"publicKey": {
		"id": "https://mastodon.social/users/Mastodon#main-key",
		"owner": "https://mastodon.social/users/Mastodon",
		"publicKeyPem": "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAtpNfuGPl/WTnSq3dTurF\nMRelAIdvGVkO/VKYZJvIleYA27/YTnpmlY2g+0az4xEhOBtVNA1cTpS63CdXRyNz\ncH/GZtzxkdxN91vZSw0JVy+wG34dzwcq1KWFDz9D/5Tqf16KUJH+TDTlxdOBds91\nIZg+TTkiT+xfnSiC5SLMnn1dTzCW9P0yNJxpn37z7p6pEs63X1wstEEX1qGDUQTO\n1JICpKDjuQZMlioAAA5KG25tg2f+zKlv5M/NI33DblquyJ7TYvIpDN8hsFCRjuvA\nmjtKz/1XIRvQkeKND3UkqX8s6qTGyNOjcT86qt9BqYHYGuppjpRG/QNGoKYalio1\nwwIDAQAB\n-----END PUBLIC KEY-----\n"
	},
	"tag": [],
	"attachment": [
		{
			"type": "PropertyValue",
			"name": "Homepage",
			"value": "<a href=\"https://joinmastodon.org\" target=\"_blank\" rel=\"nofollow noopener noreferrer me\" translate=\"no\"><span class=\"invisible\">https://</span><span class=\"\">joinmastodon.org</span><span class=\"invisible\"></span></a>"
		},
		{
			"type": "PropertyValue",
			"name": "Patreon",
			"value": "<a href=\"https://patreon.com/mastodon\" target=\"_blank\" rel=\"nofollow noopener noreferrer me\" translate=\"no\"><span class=\"invisible\">https://</span><span class=\"\">patreon.com/mastodon</span><span class=\"invisible\"></span></a>"
		},
		{
			"type": "PropertyValue",
			"name": "GitHub",
			"value": "<a href=\"https://github.com/mastodon\" target=\"_blank\" rel=\"nofollow noopener noreferrer me\" translate=\"no\"><span class=\"invisible\">https://</span><span class=\"\">github.com/mastodon</span><span class=\"invisible\"></span></a>"
		}
	],
	"endpoints": {
		"sharedInbox": "https://mastodon.social/inbox"
	},
	"icon": {
		"type": "Image",
		"mediaType": "image/png",
		"url": "https://files.mastodon.social/accounts/avatars/000/013/179/original/b4ceb19c9c54ec7e.png"
	},
	"image": {
		"type": "Image",
		"mediaType": "image/png",
		"url": "https://files.mastodon.social/accounts/headers/000/013/179/original/1375be116fbe0f1d.png"
	}
}
        "#;
        let deserialized: Result<Actor, serde_json::Error> = serde_json::from_str(mastodon_account);

        match deserialized {
            Ok(_x) => Ok(()),
            Err(x) => Err(format!("actor deserialize failed with response: {}", x)),
        }
    }
}
