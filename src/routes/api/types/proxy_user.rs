use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiProxyUser {
    pub id: Uuid,
    pub name: String,
    pub bio: Option<String>,
    pub created: i64,
    pub parent_id: Uuid,
}

impl ApiProxyUser {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            id: row.get("proxy_id"),
            name: row.get("proxy_name"),
            bio: row.get("proxy_bio"),
            created: row.get("proxy_created"),
            parent_id: row.get("uid"),
        }
    }
    pub fn maybe_from_row(row: &tokio_postgres::Row) -> Option<Self> {
        let id: Option<Uuid> = row.get("proxy_id");
        match id {
            Some(_) => Some(Self::from_row(row)),
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewProxyUser {
    pub name: String,
    pub bio: Option<String>,
}
