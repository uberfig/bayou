use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private, Public},
    rsa::Rsa,
};

use super::key::{Key, KeyType, PrivateKey, PublicKey};

#[derive(Debug, Clone)]
pub struct OpenSSLPrivate(PKey<Private>);

impl Key for OpenSSLPrivate {
    fn from_pem(pem: &[u8]) -> Result<Self, super::error::Error> {
        Ok(OpenSSLPrivate(PKey::private_key_from_pem(pem)?))
    }

    fn to_pem(&self) -> Result<String, super::error::Error> {
        let bytes = self.0.private_key_to_pem_pkcs8()?;
        let pem = String::from_utf8(bytes)?;
        Ok(pem)
    }
}

impl PrivateKey for OpenSSLPrivate {
    fn sign(&mut self, content: &str) -> String {
        let mut signer = openssl::sign::Signer::new(MessageDigest::sha256(), &self.0).unwrap();
        signer.update(content.as_bytes()).unwrap();
        openssl::base64::encode_block(&signer.sign_to_vec().unwrap())
    }

    fn generate(algorithm: KeyType) -> Self {
        match algorithm {
            KeyType::Rsa256 => {
                let rsa = Rsa::generate(2048).unwrap();
                OpenSSLPrivate(PKey::from_rsa(rsa).unwrap())
            }
            KeyType::Ed25519 => OpenSSLPrivate(openssl::pkey::PKey::generate_ed25519().unwrap()),
        }
    }

    fn public_key_pem(&self) -> Result<String, super::error::Error> {
        let bytes = self.0.public_key_to_pem()?;
        let pem = String::from_utf8(bytes)?;
        Ok(pem)
    }
}

#[derive(Debug, Clone)]
pub struct OpenSSLPublic(PKey<Public>);

impl Key for OpenSSLPublic {
    fn from_pem(pem: &[u8]) -> Result<Self, super::error::Error> {
        Ok(OpenSSLPublic(PKey::public_key_from_pem(pem)?))
    }

    fn to_pem(&self) -> Result<String, super::error::Error> {
        let bytes = self.0.public_key_to_pem()?;
        let pem = String::from_utf8(bytes)?;
        Ok(pem)
    }
}

impl PublicKey for OpenSSLPublic {
    fn verify(&self, plain_content: &[u8], signature: &[u8]) -> bool {
        let mut verifier =
            openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &self.0).unwrap();
        verifier.update(plain_content).unwrap();
        verifier.verify(signature).unwrap()
    }
}
