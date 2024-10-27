use super::super::serde_fns::{deserialize_time, serialize_time};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoteType {
    #[serde(rename = "pub.versia:polls/Vote")]
    Vote,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vote {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: VoteType,
    pub uri: Url,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,

    pub author: Url,
    pub poll: Url,
    pub option: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() -> Result<(), String> {
        //taken from the versia protocol examples
        let vote = r#"
{
    "id": "6f27bc77-58ee-4c9b-b804-8cc1c1182fa9",
    "type": "pub.versia:polls/Vote", 
    "uri": "https://example.com/actions/6f27bc77-58ee-4c9b-b804-8cc1c1182fa9",
    "created_at": "2021-01-01T00:00:00.000Z",
    "author": "https://example.com/users/6e0204a2-746c-4972-8602-c4f37fc63bbe", 
    "poll": "https://example.com/notes/f08a124e-fe90-439e-8be4-15a428a72a19",
    "option": 1
}
"#;
        let deserialized: Result<Vote, serde_json::Error> = serde_json::from_str(vote);
        match deserialized {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("poll deserialize failed: {}", x)),
        }
    }
}
