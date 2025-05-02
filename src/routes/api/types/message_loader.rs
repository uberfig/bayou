use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MessagesBefore {
    pub time: i64,
    pub post: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MessagesLoader {
    pub room: Uuid,
    pub before: Option<MessagesBefore>,
}
