use base64::Engine;
use sha2::{Digest, Sha256, Sha512};

/// generates an sha256 digest of the provided buffer encoded in base64
pub fn sha256_hash(body: &[u8]) -> String {
    let mut hasher = Sha256::new();
    // write input message
    hasher.update(body);
    let finished = hasher.finalize();
    base64::prelude::BASE64_STANDARD.encode(finished)
}

/// generates an sha512 digest of the provided buffer encoded in base64
pub fn sha512_hash(body: &[u8]) -> String {
    let mut hasher = Sha512::new();
    // write input message
    hasher.update(body);
    let finished = hasher.finalize();
    base64::prelude::BASE64_STANDARD.encode(finished)
}
