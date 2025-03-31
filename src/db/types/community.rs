use uuid::Uuid;

pub struct Community {
    pub id: Uuid,
    pub external_id: Uuid,
    pub domain: String,
    pub name: String,
    pub description: Option<String>,
    pub created: i64,
}

impl From<tokio_postgres::Row> for Community {
    fn from(row: tokio_postgres::Row) -> Self {
        Community {
            id: row.get("id"),
            external_id: row.get("external_id"),
            domain: row.get("domain"),
            name: row.get("name"),
            description: row.get("description"),
            created: row.get("created"),
        }
    }
}
