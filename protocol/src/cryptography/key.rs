use serde::{Deserialize, Serialize};

use super::{digest::{sha256_hash, sha512_hash}, error::Error};

#[derive(Debug, Clone)]
pub enum ParseErr {
    Failure,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Algorithms {
    #[serde(rename = "rsa-sha256")]
    RsaSha256,
    /// is actually Ed25519-SHA512
    #[serde(rename = "hs2019")]
    Hs2019,
}

impl Algorithms {
    /// hash a body with the respective hashing algorithm and outputs `SHA-__=hash`
    pub fn hash(&self, body: &[u8]) -> String {
        match self {
            Algorithms::RsaSha256 => format!("SHA-256={}", sha256_hash(body)),
            Algorithms::Hs2019 => format!("SHA-512={}", sha512_hash(body)),
        }
    }
}

impl std::fmt::Display for Algorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithms::RsaSha256 => write!(f, "rsa-sha256"),
            Algorithms::Hs2019 => write!(f, "hs2019"),
        }
    }
}

pub trait Key: Sized {
    /// Serialize from PEM
    fn from_pem(pem: &[u8]) -> Result<Self, Error>;
    /// Serialize self to PEM.
    /// if a public key this will be the public pem
    fn to_pem(&self) -> Result<String, Error>;
}

pub trait PrivateKey: Key + Clone {
    /// sign the provided content with this key
    fn sign(&mut self, content: &str) -> String;
    // fn from_pem(pem: &str, algorithm: crate::cryptography::key::KeyType) -> Result<Self, ParseErr>;
    fn generate(algorithm: Algorithms) -> Self;
    // fn private_key_pem(&self) -> String;
    fn public_key_pem(&self) -> Result<String, Error>;
}

pub trait PublicKey: Key + Clone {
    /// verify that the provided content was signed with this key
    fn verify(&self, plain_content: &[u8], signature: &[u8]) -> bool;
    // fn from_pem(pem: &str, algorithm: KeyType) -> Result<Self, ParseErr>;
}
