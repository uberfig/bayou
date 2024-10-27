use serde::{Deserialize, Serialize};

use super::error::Error;

#[derive(Debug, Clone)]
pub enum ParseErr {
    Failure,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyType {
    Rsa256,
    Ed25519,
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
    fn generate(algorithm: KeyType) -> Self;
    // fn private_key_pem(&self) -> String;
    fn public_key_pem(&self) -> Result<String, Error>;
}

pub trait PublicKey: Key + Clone {
    /// verify that the provided content was signed with this key
    fn verify(&self, plain_content: &[u8], signature: &[u8]) -> bool;
    // fn from_pem(pem: &str, algorithm: KeyType) -> Result<Self, ParseErr>;
}
