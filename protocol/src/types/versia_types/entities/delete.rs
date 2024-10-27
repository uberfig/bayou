use super::super::serde_fns::{deserialize_time, serialize_time};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteTypeField {
    Delete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeletedType {
    Note,
    User,
    Share,
}

/// Signals the deletion of an entity.
///
/// Implementations must ensure that the author of the Delete entity has the authorization to delete the target entity.
/// - The author is the creator of the target entity (including [delegation](https://versia.pub/federation/delegation)).
/// - The author is the instance.
///
/// https://versia.pub/entities/delete
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delete {
    #[serde(rename = "type")]
    pub type_field: DeleteTypeField,
    pub id: String,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,
    /// URI of the User who is deleting the entity. Can be set to null to represent the instance.
    ///
    /// https://versia.pub/entities/instance-metadata#the-null-author
    pub author: Option<Url>,
    pub deleted_type: DeletedType,
    pub deleted: Url,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() -> Result<(), String> {
        //taken from the versia protocol examples
        let delete = r#"
{
    "type": "Delete",
    "id": "9b3212b8-529c-435a-8798-09ebbc17ca74",
    "created_at": "2021-01-01T00:00:00.000Z",
    "author": "https://example.com/users/6e0204a2-746c-4972-8602-c4f37fc63bbe",
    "deleted_type": "Note",
    "deleted": "https://example.com/notes/02e1e3b2-cb1f-4e4a-b82e-98866bee5de7"
}
"#;
        let deserialized: Result<Delete, serde_json::Error> = serde_json::from_str(delete);
        match deserialized {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("delete deserialize failed: {}", x)),
        }
    }
}
