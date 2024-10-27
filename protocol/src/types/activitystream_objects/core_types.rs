use serde::{Deserialize, Serialize};

//--------------primitive-----------------

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// represents a field that could be a single item or array of items
pub enum OptionalArray<T> {
    Single(T),
    Multiple(Vec<T>),
}
