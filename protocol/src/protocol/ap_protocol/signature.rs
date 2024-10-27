// borrows heavily from https://github.com/astro/sigh/blob/main/src/signature.rs

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

use super::super::{headers::Headers, http_method::HttpMethod};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Algorithms {
    #[serde(rename = "rsa-sha256")]
    RsaSha256,
    /// is actually Ed25519-SHA512
    #[serde(rename = "hs2019")]
    Hs2019,
}

impl std::fmt::Display for Algorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithms::RsaSha256 => write!(f, "rsa-sha256"),
            Algorithms::Hs2019 => write!(f, "hs2019"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureErr {
    NoSignature,
    NoKey,
    InvalidKey,
    InvalidDomain,
    NoHeaders,
    UnkownAlgorithm,
    MissingHeader(String),
    NoDate,
    BadSignature,
}

/// A parsed representation of the `Signature:` header
///
/// `keyId="https://my.example.com/actor#main-key",headers="(request-target) host date",signature="Y2FiYW...IxNGRiZDk4ZA=="`
#[derive(Debug, Clone)]
pub struct SignatureHeader {
    pub headers: Vec<String>,
    /// defaults to rsa-sha256 if not present
    pub algorithm: Algorithms,
    pub key_id: Url,
    pub key_domain: String,
    /// the contained signature
    pub signature: Vec<u8>,
}

impl SignatureHeader {
    /// parse the signature header
    pub fn parse(signature: &str) -> Result<Self, SignatureErr> {
        let signature_headers = get_signature_headers(signature);
        let Some(key_id) = signature_headers.get("keyId") else {
            return Err(SignatureErr::NoKey);
        };
        let key_id = key_id.replace('"', "");
        let Ok(key_id) = Url::parse(&key_id) else {
            return Err(SignatureErr::InvalidKey);
        };
        let Some(key_domain) = key_id.domain() else {
            return Err(SignatureErr::InvalidDomain);
        };
        let key_domain = key_domain.to_string();

        let Some(signature) = signature_headers.get("signature") else {
            return Err(SignatureErr::NoSignature);
        };

        let Some(headers) = signature_headers.get("headers") else {
            return Err(SignatureErr::NoHeaders);
        };

        let algorithm = signature_headers.get("algorithm");
        let algorithm = match algorithm {
            Some(x) => {
                let val: Result<Algorithms, serde_json::Error> = serde_json::from_str(x);
                match val {
                    Ok(ok) => ok,
                    Err(_) => return Err(SignatureErr::UnkownAlgorithm),
                }
            }
            None => Algorithms::RsaSha256,
        };

        let headers: Vec<String> = headers.split_ascii_whitespace().map(String::from).collect();

        let Ok(signature) = openssl::base64::decode_block(signature) else {
            return Err(SignatureErr::BadSignature);
        };

        Ok(Self {
            headers,
            algorithm,
            key_id,
            key_domain,
            signature,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub method: HttpMethod,
    /// the domain that is hosting the resource
    pub host: String,
    /// the path of the request
    pub request_target: String,
    /// A parsed representation of the `Signature:` header
    pub signature_header: SignatureHeader,
}

impl Signature {
    pub fn from_request(
        method: HttpMethod,
        host: String,
        request_target: String,
        signature_header: &str,
    ) -> Result<Self, SignatureErr> {
        Ok(Signature {
            method,
            host,
            request_target,
            signature_header: SignatureHeader::parse(signature_header)?,
        })
    }
    pub fn generate_sign_string<H: Headers>(
        &self,
        request_headers: &H,
    ) -> Result<String, SignatureErr> {
        let headers = self
            .signature_header
            .headers
            .iter()
            .map(std::ops::Deref::deref);
        let mut comparison_string: Vec<String> = Vec::new();
        for signed_header_name in headers {
            match signed_header_name {
                "(request-target)" => comparison_string.push(format!(
                    "(request-target): {} {}",
                    self.method.stringify(),
                    &self.request_target
                )),
                "host" => comparison_string.push(format!("host: {}", &self.host)),
                _ => {
                    let Some(value) = request_headers.get(signed_header_name) else {
                        return Err(SignatureErr::MissingHeader(signed_header_name.to_string()));
                    };
                    let x = format!("{signed_header_name}: {value}",);
                    comparison_string.push(x);
                }
            }
        }

        Ok(comparison_string.join("\n"))
    }
}

pub fn get_signature_headers(signature_header: &str) -> HashMap<String, String> {
    signature_header
        .split(',')
        .filter_map(|pair| {
            pair.split_once('=').map(|(key, value)| {
                (
                    key.replace(|c| strip_values(c), ""),
                    value.replace(|c| strip_values(c), ""),
                )
            })
        })
        .collect()
}

fn strip_values(c: char) -> bool {
    c.eq(&'"')
    // || c.eq(&' ')
}

#[cfg(test)]
mod tests {
    use super::super::super::headers::HashMapHeaders;

    use super::*;

    #[test]
    fn parse_get() -> Result<(), String> {
        let signature = match Signature::from_request(
            HttpMethod::Get,
            "mastodon.example".to_string(),
            "/users/username/outbox".to_string(),
            r#"keyId="https://my.example.com/actor#main-key",headers="(request-target) host date",signature="Y2FiYWIxNGRiZDk4ZA==""#,
        ) {
            Ok(x) => x,
            Err(x) => return Err(serde_json::to_string_pretty(&x).unwrap()),
        };

        dbg!(&signature);

        let mut hashmap = HashMap::new();
        hashmap.insert("host".to_string(), "mastodon.example".to_string());
        hashmap.insert("date".to_string(), "18 Dec 2019 10:08:46 GMT".to_string());
        let request_headers = HashMapHeaders { headermap: hashmap };

        let signed_string = match signature.generate_sign_string(&request_headers) {
            Ok(ok) => ok,
            Err(err) => return Err(serde_json::to_string_pretty(&err).unwrap()),
        };

        let correct = "(request-target): get /users/username/outbox\nhost: mastodon.example\ndate: 18 Dec 2019 10:08:46 GMT";
        if signed_string.ne(&correct) {
            return Err(format!("bad sign string: {}", signed_string));
        }
        Ok(())
    }
}
