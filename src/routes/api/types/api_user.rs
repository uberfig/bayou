use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiUser {
    pub id: Uuid,
    pub domain: String,
    pub username: String,
    pub display_name: Option<String>,
    pub summary: Option<String>,
    pub created: i64,
}
