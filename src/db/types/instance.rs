pub struct Instance {
    pub domain: String,
    pub is_authoratative: bool,
    pub blocked: bool,
    pub reason: Option<String>,
    pub allowlisted: bool,
}

impl From<tokio_postgres::Row> for Instance {
    fn from(row: tokio_postgres::Row) -> Self {
        Instance {
            domain: row.get("domain"),
            is_authoratative: row.get("is_authoratative"),
            blocked: row.get("blocked"),
            reason: row.get("reason"),
            allowlisted: row.get("allowlisted"),
        }
    }
}
