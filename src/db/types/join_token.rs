use uuid::Uuid;

pub struct JoinToken {
    pub id: Uuid,
    pub creator: Uuid,
    pub commmunity: Uuid,
    pub expiry: i64,
}

impl From<tokio_postgres::Row> for JoinToken {
    fn from(row: tokio_postgres::Row) -> Self {
        JoinToken {
            id: row.get("token_id"),
            creator: row.get("creator"),
            commmunity: row.get("commmunity"),
            expiry: row.get("expiry"),
        }
    }
}

impl JoinToken {
    pub const fn create_statement() -> &'static str {
        r#"
        INSERT INTO join_token
        (token_id, creator, commmunity, expiry)
        VALUES
        ($1, $2, $3, $4)
        RETURNING *;
        "#
    }
    pub const fn read_statement() -> &'static str {
        r#"
        SELECT * FROM join_token WHERE token_id = $1;
        "#
    }
    pub const fn delete_statement() -> &'static str {
        r#"
        DELETE FROM join_token WHERE token_id = $1;
        "#
    }
}
