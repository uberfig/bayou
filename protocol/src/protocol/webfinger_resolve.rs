use reqwest::StatusCode;
use url::Url;

use crate::protocol::{
    errors::FetchErr,
    webfinger::RelWrap,
};

use super::webfinger::WebfingerResult;

#[derive(Debug, Clone)]
pub struct WebfingerInfo {
    pub page: Url,
    pub activitypub_item: Url,
}

pub async fn webfinger_resolve(
    username: &str,
    domain: &str,
    rel: RelWrap,
) -> Result<Url, FetchErr> {
    let query = format!("https://{domain}/.well-known/webfinger?resource=acct:{username}@{domain}");
    let query = Url::parse(&query).expect("generated invalid url for webfinger resolve");

    let client = reqwest::Client::new();
    let client = client
        .get(query)
        // .header("User-Agent", value)
        .body("");

    let res = client.send().await;
    let res = match res {
        Ok(ok) => ok,
        Err(err) => return Err(FetchErr::RequestErr(err.to_string())),
    };

    match res.status() {
        StatusCode::OK => {
            let body = res.text().await.unwrap_or("".to_string());
            let val: Result<WebfingerResult, serde_json::Error> = serde_json::from_str(&body);
            let val = match val {
                Ok(ok) => ok,
                Err(err) => return Err(FetchErr::DeserializationErr(err.to_string())),
            };
            for link in val.links {
                if link.rel == rel {
                    return Ok(link.href);
                }
            }
            return Err(FetchErr::MissingField(rel.to_string()));
        }
        StatusCode::NOT_FOUND => Err(FetchErr::NotFound()),
        _ => Err(FetchErr::RequestErr(format!(
            "post got status: {} with body: {}",
            res.status(),
            res.text().await.unwrap_or("".to_string())
        ))),
    }
}
