use uuid::Uuid;

pub struct Room {
    pub id: Uuid,
    pub external_id: Uuid,
    pub domain: String,
    pub community: Option<Uuid>,
    pub category: Option<Uuid>,
    pub display_order: i64,
    pub name: String,
    pub description: Option<String>,
    pub created: i64,
    pub is_dm: bool,
    /// if this is a dm or group chat this is the user that started it
    /// we do this so that it can be automatically deleted if that user
    /// is deleted and we are able to query for dms
    pub user_a: Option<Uuid>,
    /// only used for direct messages, user B will not be the one to
    /// init the chat. exists so it will be auto deleted if they
    /// are deleted and so they can query for dms
    pub user_b: Option<Uuid>,
}

impl From<tokio_postgres::Row> for Room {
    fn from(row: tokio_postgres::Row) -> Self {
        Room {
            id: row.get("id"),
            external_id: row.get("external_id"),
            domain: row.get("domain"),
            community: row.get("community"),
            category: row.get("category"),
            display_order: row.get("display_order"),
            name: row.get("name"),
            description: row.get("description"),
            created: row.get("created"),
            is_dm: row.get("is_dm"),
            user_a: row.get("user_a"),
            user_b: row.get("user_b"),
        }
    }
}
