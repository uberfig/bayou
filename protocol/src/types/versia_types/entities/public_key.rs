use base64::Engine;
// use ed25519::pkcs8::DecodePublicKey;
// use ed25519::Signature;
// use ed25519_dalek::Verifier;
use serde::{de::Error as DeError, Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use url::Url;

/// The user's public key. Must follow the Versia Public Key format.
/// actor may be a URI to another user's profile, in which case this
/// key may allow the other user act on behalf of this user (see delegation).
/// - algorithm: Must be ed25519 for now.
/// - key: The public key in SPKI-encoded base64 (from raw bytes, not a PEM format). Must be the key associated with the actor URI.
/// - actor: URI to a user's profile, most often the user's own profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersiaPublicKey {
    pub actor: Option<Url>,
    /// algorithm used for the public key. Can only be ed25519 for now.
    ///
    /// public key, in SPKI-encoded base64 (from raw bytes, not a PEM format).
    #[serde(flatten)]
    pub key: AlgorithmsPublicKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "algorithm", content = "key")]
pub enum AlgorithmsPublicKey {
    #[serde(rename = "ed25519")]
    Ed25519(Ed25519Public),
}

// impl crate::cryptography::key::PublicKey for AlgorithmsPublicKey {
//     fn verify(&self, plain_content: &str, signature: &str) -> bool {
//         match self {
//             AlgorithmsPublicKey::Ed25519(ed25519_public) => {
//                 let Ok(signature) = Signature::from_slice(signature.as_bytes()) else {
//                     return false;
//                 };
//                 ed25519_public
//                     .key
//                     .verify(plain_content.as_bytes(), &signature)
//                     .is_ok()
//             }
//         }
//     }

//     fn from_pem(
//         pem: &str,
//         algorithm: crate::cryptography::key::KeyType,
//     ) -> Result<Self, crate::cryptography::key::ParseErr> {
//         match algorithm {
//             crate::cryptography::key::KeyType::Ed25519 => {
//                 let Ok(val) = ed25519_dalek::VerifyingKey::from_public_key_pem(pem) else {
//                     return Err(crate::cryptography::key::ParseErr::Failure);
//                 };
//                 Ok(AlgorithmsPublicKey::Ed25519(Ed25519Public { key: val }))
//             }
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Ed25519Public {
    pub key: ed25519_dalek::VerifyingKey,
}

impl Serialize for Ed25519Public {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let binary = self.key.as_bytes();
        let base64_val = base64::prelude::BASE64_STANDARD.encode(binary);
        serializer.serialize_str(&base64_val)
    }
}

impl<'de> Deserialize<'de> for Ed25519Public {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = <&str>::deserialize(deserializer)?;
        let binary = match base64::prelude::BASE64_STANDARD.decode(input) {
            Ok(ok) => ok,
            Err(err) => return Err(D::Error::custom(err)),
        };
        let Ok(binary) = binary.try_into() else {
            return Err(D::Error::custom("invalid binary length"));
        };
        let key = ed25519_dalek::VerifyingKey::from_bytes(&binary);
        let key = match key {
            Ok(ok) => ok,
            Err(err) => return Err(D::Error::custom(err)),
        };
        Ok(Ed25519Public { key })
    }
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    use super::super::super::entities::public_key::Ed25519Public;

    use super::*;

    fn generate_verifying_key() -> ed25519_dalek::VerifyingKey {
        let mut csprng = OsRng;
        let key = SigningKey::generate(&mut csprng);
        key.verifying_key()
    }

    #[test]
    fn test_serialize() -> Result<(), String> {
        let key = Ed25519Public {
            key: generate_verifying_key(),
        };
        let key = VersiaPublicKey {
            actor: None,
            key: AlgorithmsPublicKey::Ed25519(key),
        };
        let deserized = serde_json::to_string(&key);
        match deserized {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("deserialize failure: {}", x)),
        }
    }

    #[test]
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
        let public_key = format!(
            r#"
{{
    "actor": "https://versia.social/users/018ec082-0ae1-761c-b2c5-22275a611771",
    "algorithm": "ed25519",
    "key": {}
}}
        "#,
            key
        );
        println!("{}", &public_key);
        let deserialized: Result<VersiaPublicKey, serde_json::Error> =
            serde_json::from_str(&public_key);
        match deserialized {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("user deserialize failed: {}", x)),
        }
    }
}
