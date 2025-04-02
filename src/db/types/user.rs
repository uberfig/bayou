use serde::{Deserialize, Serialize};
use std::time::{self, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::cryptography::passwords::hash_password;

pub struct DbUser {
    pub id: Uuid,
    pub info: UserInfo,
}

pub struct UserInfo {
    pub domain: String,
    pub username: String,
    pub display_name: Option<String>,
    pub summary: Option<String>,
    pub custom_emoji: Option<String>,
    pub banned: bool,
    pub reason: Option<String>,
    pub fetched_at: Option<i64>,
    pub created: i64,
    pub local_info: Option<LocalUser>,
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
    pub application_approved: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SignupResult {
    UsernameTaken,
    InvalidToken,
    Success,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupUser {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub token: Option<String>,
    pub application_message: Option<String>,
}

impl SignupUser {
    pub fn into_user(self, instance_domain: &str) -> UserInfo {
        // yes yes we are downcasting to an i64, if this is somehow still used
        // in 500 years then peeps can just use seconds instead of milis
        // or just upgrade to i128 or whatever they use in 500 years
        let curr_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_millis() as i64;

        UserInfo {
            domain: instance_domain.to_string(),
            username: self.username,
            display_name: None,
            summary: None,
            custom_emoji: None,
            banned: false,
            reason: None,
            fetched_at: None,
            created: curr_time,
            local_info: Some(LocalUser {
                password: hash_password(self.password.as_bytes()),
                email: self.email,
                verified: false,
                is_admin: false,
                instance_mod: false,
                application_message: self.application_message,
                application_approved: false,
            }),
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
                domain: row.get("domain"),
                username: row.get("username"),
                display_name: row.get("display_name"),
                summary: row.get("summary"),
                custom_emoji: row.get("custom_emoji"),
                banned: row.get("banned"),
                reason: row.get("reason"),
                fetched_at: row.get("fetched_at"),
                local_info,
                created: row.get("created"),
            },
        }
    }
}
