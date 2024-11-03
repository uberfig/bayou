use openssl::{
    pkey::{PKey, Private, Public},
    rsa::Rsa,
};

use super::key::{Algorithms, Key, PrivateKey, PublicKey};

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
        let mut signer = openssl::sign::Signer::new_without_digest(&self.0).unwrap();
        let test = signer.sign_oneshot_to_vec(content.as_bytes()).unwrap();
        openssl::base64::encode_block(&test)
    }

    fn generate(algorithm: Algorithms) -> Self {
        match algorithm {
            Algorithms::RsaSha256 => {
                let rsa = Rsa::generate(2048).unwrap();
                OpenSSLPrivate(PKey::from_rsa(rsa).unwrap())
            }
            Algorithms::Hs2019 => OpenSSLPrivate(openssl::pkey::PKey::generate_ed25519().unwrap()),
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
        let mut verifier = openssl::sign::Verifier::new_without_digest(&self.0).unwrap();
        verifier.verify_oneshot(signature, plain_content).unwrap()
    }
}
