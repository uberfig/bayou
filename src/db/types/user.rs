use uuid::Uuid;

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
