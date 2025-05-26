use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::types::comm::community::DbCommunity;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiCommunity {
    pub id: Uuid,
    pub domain: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: Uuid,
    pub created: i64,
}

impl From<DbCommunity> for ApiCommunity {
    fn from(value: DbCommunity) -> Self {
        Self {
            id: value.id,
            domain: value.domain,
            name: value.info.name,
            description: value.info.description,
            owner: value.owner,
            created: value.created,
        }
    }
}
