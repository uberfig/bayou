use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DBAuthToken {
    pub required_token: AuthToken,
    pub expiry: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthToken {
    #[serde(with = "uuid::serde::simple")]
    pub token: Uuid,
    /// auth tokens will only be valid for the device they were
    /// issued to
    #[serde(with = "uuid::serde::simple")]
    pub device_id: Uuid,
    /// the uid this auth token is valid for
    #[serde(with = "uuid::serde::simple")]
    pub uid: Uuid,
}

impl From<tokio_postgres::Row> for DBAuthToken {
    fn from(row: tokio_postgres::Row) -> Self {
        DBAuthToken {
            required_token: AuthToken {
                token: row.get("token_id"),
                device_id: row.get("device_id"),
                uid: row.get("uid"),
            },
            expiry: row.get("expiry"),
        }
    }
}

impl DBAuthToken {
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
