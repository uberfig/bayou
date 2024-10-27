use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
/// wraps base object to include context
pub struct ContextWrap<T: Clone> {
    #[serde(rename = "@context")]
    pub context: Context,
    #[serde(flatten)]
    pub item: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Context {
    Array(Vec<ContextItem>),
    Single(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContextItem {
    String(String),
    Map(HashMap<String, ContextMapItem>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContextMapItem {
    String(String),
    Map(HashMap<String, String>),
}
