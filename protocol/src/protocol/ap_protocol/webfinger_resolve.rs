use url::Url;

#[derive(Debug, Clone)]
pub struct WebfingerResult{
    pub page: Url,
    pub activitypub_item: Url,
}

pub async fn webfinger_resolve(username: &str, domain: &str) -> WebfingerResult {
    let query = format!("https://{domain}/.well-known/webfinger?resource=acct:{username}@{domain}");
    let query = Url::parse(&query).expect("generated invalid url for webfinger resolve");

    let client = reqwest::Client::new();
    let client = client
        .get(query)
        // .header("User-Agent", value)
        .body("");

    let res = client.send().await;


    todo!()
}