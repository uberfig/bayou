use uuid::Uuid;

pub struct SignupToken {
    pub id: Uuid,
    pub creator: Uuid,
    pub expiry: i64,
}

impl From<tokio_postgres::Row> for SignupToken {
    fn from(row: tokio_postgres::Row) -> Self {
        SignupToken {
            id: row.get("token_id"),
            creator: row.get("creator"),
            expiry: row.get("expiry"),
        }
    }
}
