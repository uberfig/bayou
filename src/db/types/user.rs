use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::api::types::api_user::ApiUser;

pub struct DbUser {
    pub id: Uuid,
    pub info: UserInfo,
    pub local_info: Option<LocalUser>,
    pub fetched_at: Option<i64>,
    pub domain: String,
    pub banned: bool,
    pub reason: Option<String>,
}

impl From<DbUser> for ApiUser {
    fn from(user: DbUser) -> Self {
        ApiUser {
            id: user.id,
            domain: user.domain,
            username: user.info.username,
            display_name: user.info.display_name,
            summary: user.info.summary,
            created: user.info.created,
        }
    }
}

pub struct UserInfo {
    pub username: String,
    pub display_name: Option<String>,
    pub summary: Option<String>,
    pub created: i64,
}

pub struct LocalUser {
    /// argon2 hash
    pub password: String,
    pub email: Option<String>,
    pub verified: bool,
    pub is_admin: bool,
    pub instance_mod: bool,
    /// used for if signups require an application
    pub application_message: Option<String>,
    pub application_approved: Option<bool>,
}

impl From<tokio_postgres::Row> for ApiUser {
    fn from(row: tokio_postgres::Row) -> Self {
        ApiUser {
            id: row.get("uid"),
            domain: row.get("domain"),
            username: row.get("username"),
            display_name: row.get("display_name"),
            summary: row.get("summary"),
            created: row.get("created"),
        }
    }
}

impl From<tokio_postgres::Row> for DbUser {
    fn from(row: tokio_postgres::Row) -> Self {
        let is_authoratative: bool = row.get("is_authoratative");
        let local_info = match is_authoratative {
            true => Some(LocalUser {
                password: row.get("password"),
                email: row.get("email"),
                verified: row.get("verified"),
                is_admin: row.get("is_admin"),
                instance_mod: row.get("instance_mod"),
                application_message: row.get("application_message"),
                application_approved: row.get("application_approved"),
            }),
            false => None,
        };
        DbUser {
            id: row.get("uid"),
            info: UserInfo {
                username: row.get("username"),
                display_name: row.get("display_name"),
                summary: row.get("summary"),
                created: row.get("created"),
            },
            local_info,
            fetched_at: row.get("fetched_at"),
            domain: row.get("domain"),
            banned: row.get("banned"),
            reason: row.get("reason"),
        }
    }
}

impl DbUser {
    pub const fn create_statement() -> &'static str {
        r#"
        INSERT INTO users 
        (
            uid,
            domain,
            username,
            display_name,
            summary,
            banned,
            reason,
            fetched_at,
            is_authoratative,
            password,
            email,
            verified,
            is_admin,
            instance_mod,
            application_message,
            application_approved,
            created
        )
        VALUES
        (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 
            $11, $12, $13, $14, $15, $16, $17
        )
        RETURNING *;
        "#
    }
    pub const fn read_uid_statement() -> &'static str {
        r#"
        SELECT * FROM users WHERE uid = $1;
        "#
    }
    pub const fn read_statement() -> &'static str {
        r#"
        SELECT * FROM users WHERE username = $1 AND domain = $2;
        "#
    }
    pub const fn update_statement() -> &'static str {
        r#"
        UPDATE users SET
        display_name = $1,
        summary = $2,
        instance_mod = $3,
        banned = $4,
        reason = $5,
        fetched_at = $6,
        is_authoratative = $7,
        password = $8,
        email = $9,
        verified = $10,
        is_admin = $11,
        
        WHERE uid = $14
        RETURNING *;
        "#
    }
    pub const fn delete_statement() -> &'static str {
        r#"
        DELETE FROM users WHERE uid = $1;
        "#
    }
}
