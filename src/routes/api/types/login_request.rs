use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    #[serde(with = "uuid::serde::simple")]
    pub device_id: Uuid,
}
