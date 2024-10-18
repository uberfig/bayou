use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Page {
    pub page: Option<u64>,
    pub is_page: Option<bool>,
}
