use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthToken {
    pub token: Uuid,
    pub device_id: Uuid,
    pub uid: Uuid,
    pub expiry: i64,
}

impl From<tokio_postgres::Row> for AuthToken {
    fn from(row: tokio_postgres::Row) -> Self {
        AuthToken {
            token: row.get("token_id"),
            device_id: row.get("device_id"),
            uid: row.get("uid"),
            expiry: row.get("expiry"),
        }
    }
}

impl AuthToken {
    pub const fn create_statement() -> &'static str {
        r#"
        INSERT INTO auth_tokens
        (token_id, device_id, uid, expiry)
        VALUES
        ($1, $2, $3, $4)
        RETURNING *;
        "#
    }
    pub const fn read_statement() -> &'static str {
        r#"
        SELECT * FROM auth_tokens WHERE token_id = $1;
        "#
    }
    pub const fn delete_statement() -> &'static str {
        r#"
        DELETE FROM auth_tokens WHERE token_id = $1;
        "#
    }
}
