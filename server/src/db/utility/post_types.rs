use bayou_protocol::types::activitystream_objects::{object::Object, question::Question};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostSupertype {
    Object,
    Question,
}
impl PostSupertype {
    pub fn parse_str(value: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(value)
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
/// a concrete post to be stored in the database.
/// surtype of either object or question, then subtypes of their
/// respective types, eg note, or for a question multi or single select
pub enum PostType {
    Object(Object),
    Question(Question),
}
impl PostType {
    // pub fn to_create_activitystream(self) -> ActivityStream {
    //     match self {
    //         PostType::Object(x) => x.to_create_activitystream(),
    //         PostType::Question(_) => todo!(),
    //     }
    // }
    pub fn get_surtype(&self) -> String {
        match self {
            PostType::Object(_) => serde_json::to_string(&PostSupertype::Object).unwrap(),
            PostType::Question(_) => serde_json::to_string(&PostSupertype::Question).unwrap(),
        }
    }
    pub fn get_subtype(&self) -> String {
        match self {
            PostType::Object(x) => serde_json::to_string(&x.type_field).unwrap(),
            PostType::Question(x) => serde_json::to_string(&x.type_field).unwrap(),
        }
    }
    // pub fn get_published(&self) -> &Option<String> {
    //     match self {
    //         PostType::Object(x) => &x.object.published,
    //         PostType::Question(_) => todo!(),
    //     }
    // }
    // pub fn get_id(&self) -> &str {
    //     match self {
    //         PostType::Object(x) => x.get_id().as_str(),
    //         PostType::Question(_) => todo!(),
    //     }
    // }
}

impl From<PostType> for String {
    fn from(value: PostType) -> Self {
        match value {
            PostType::Object(_) => "Object".to_string(),
            PostType::Question(_) => "Question".to_string(),
        }
    }
}
