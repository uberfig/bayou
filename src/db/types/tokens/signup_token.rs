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

impl SignupToken {
    pub const fn create_statement() -> &'static str {
        r#"
        INSERT INTO signup_token
        (token_id, creator, expiry)
        VALUES
        ($1, $2, $3)
        RETURNING *;
        "#
    }
    pub const fn read_statement() -> &'static str {
        r#"
        SELECT * FROM signup_token WHERE token_id = $1;
        "#
    }
    pub const fn delete_statement() -> &'static str {
        r#"
        DELETE FROM signup_token WHERE token_id = $1;
        "#
    }
}
