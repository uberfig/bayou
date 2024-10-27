use super::super::http_method::HttpMethod;

pub fn signature_string(
    method: HttpMethod,
    path: &str,
    nonce: &str,
    hash: &str,
    // currently versia does not include timestamps but it will in the future
    // once its added to the protocol, we should just need to add _timestamp
    // the _timestamp field is here to ensure a smooth transition for when its time
    _timestamp: i64,
) -> String {
    format!(
        "{} {} {} {}",
        method.stringify(),
        path,
        nonce,
        hash,
        // timestamp
    )
}
