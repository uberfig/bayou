use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media<T> {
    pub description: Option<String>,
    pub media_type: T,
    pub url: Url,
}