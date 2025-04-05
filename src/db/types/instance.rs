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

impl Instance {
    pub const fn create_statement() -> &'static str {
        r#"
        INSERT INTO instances
        (domain, is_authoratative, blocked, reason, allowlisted)
        VALUES
        ($1, $2, $3, $4, $5)
        RETURNING *;
        "#
    }
    pub const fn read_statement() -> &'static str {
        r#"
        SELECT * FROM instances WHERE domain = $1;
        "#
    }
    pub const fn update_statement() -> &'static str {
        r#"
        UPDATE instances SET
        is_authoratative = $1,
        blocked = $2,
        reason = $3,
        allowlisted = $4
        WHERE domain = $5
        RETURNING *;
        "#
    }
    pub const fn delete_statement() -> &'static str {
        r#"
        DELETE FROM instances WHERE domain = $1;
        "#
    }
}
