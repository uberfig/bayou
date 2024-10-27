use super::{
    signatures::signature_string,
    verify::{verify_request, VersiaVerificationCache},
};
use crate::{
    cryptography::{digest, key::PrivateKey},
    protocol::{errors::FetchErr, headers::ReqwestHeaders, http_method::HttpMethod},
};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use textnonce::TextNonce;
use url::Url;

pub enum Signer {
    User(Url),
    Instance(String),
}
impl Signer {
    pub fn domain(&self) -> Option<&str> {
        match self {
            Signer::User(url) => url.domain(),
            Signer::Instance(x) => Some(x),
        }
    }
}
impl std::fmt::Display for Signer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signer::User(url) => write!(f, "{}", url),
            Signer::Instance(x) => write!(f, "{}", x),
        }
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SOFTWARE_NAME: &str = env!("CARGO_PKG_NAME");

pub async fn versia_fetch<T: for<'a> Deserialize<'a>, K: PrivateKey, V: VersiaVerificationCache>(
    target: Url,
    mut signing_key: K,
    signed_by: &Signer,
    conn: &V,
) -> Result<T, FetchErr> {
    let nonce = TextNonce::new().into_string();
    let path = target.path();
    let hash = digest::sha256_hash("".as_bytes());
    let signed_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let signature_string = signature_string(HttpMethod::Get, path, &nonce, &hash, signed_at);
    let signature = signing_key.sign(&signature_string).to_string();

    let client = reqwest::Client::new();
    let client = client
        .get(target.clone())
        .header("Accept", "application/json")
        .header("X-Signature", signature)
        .header("X-Signed-By", signed_by.to_string())
        .header("X-Nonce", nonce)
        .header("User-Agent", format!("{}/{}", SOFTWARE_NAME, VERSION))
        .header("Signed-milis", signed_at)
        .body("");

    let res = client.send().await;

    let res = match res {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    };

    let headers = ReqwestHeaders {
        headermap: res.headers().clone(),
    };

    let response = res.text().await;
    let response = match response {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    };

    let hash = digest::sha256_hash(response.as_bytes());

    if let Err(val) = verify_request(&headers, HttpMethod::Get, path, &hash, conn).await {
        return Err(FetchErr::VerifyErr(val));
    }

    let object: Result<T, serde_json::Error> = serde_json::from_str(&response);
    let object = match object {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::DeserializationErr(x.to_string())),
    };

    Ok(object)
}

pub async fn versia_post<K: PrivateKey, V: VersiaVerificationCache>(
    target: Url,
    content: &str,
    mut signing_key: K,
    signed_by: &Signer,
    conn: &V,
) -> Result<(), FetchErr> {
    let nonce = TextNonce::new().into_string();
    let path = target.path();
    let hash = digest::sha256_hash(content.as_bytes());
    let signed_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let signature_string = signature_string(HttpMethod::Post, path, &nonce, &hash, signed_at);
    let signature = signing_key.sign(&signature_string).to_string();

    let client = reqwest::Client::new();
    let client = client
        .post(target.clone())
        .header("Accept", "application/json")
        .header("X-Signature", signature)
        .header("X-Signed-By", signed_by.to_string())
        .header("X-Nonce", nonce)
        .header("User-Agent", format!("{}/{}", SOFTWARE_NAME, VERSION))
        .header("Signed-milis", signed_at)
        .body(content.to_owned());

    let res = client.send().await;

    let res = match res {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    };

    let headers = ReqwestHeaders {
        headermap: res.headers().clone(),
    };

    let response = res.text().await;
    let response = match response {
        Ok(x) => x,
        Err(x) => return Err(FetchErr::RequestErr(x.to_string())),
    };

    let hash = digest::sha256_hash(response.as_bytes());

    if let Err(val) = verify_request(&headers, HttpMethod::Get, path, &hash, conn).await {
        return Err(FetchErr::VerifyErr(val));
    }

    Ok(())
}
