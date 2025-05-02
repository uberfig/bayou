use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    cryptography::passwords::hash_password,
    db::{
        curr_time::get_current_time,
        types::user::{DbUser, LocalUser, UserInfo},
    },
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupUser {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub token: Option<Uuid>,
    pub application_message: Option<String>,
}

impl SignupUser {
    pub fn into_user(self, instance_domain: &str) -> DbUser {
        let curr_time = get_current_time();
        let id = Uuid::now_v7();

        DbUser {
            id,
            info: UserInfo {
                username: self.username,
                display_name: None,
                summary: None,
                created: curr_time,
            },
            local_info: Some(LocalUser {
                password: hash_password(self.password.as_bytes()),
                email: self.email,
                verified: false,
                is_admin: false,
                instance_mod: false,
                application_message: self.application_message,
                application_approved: Some(false),
            }),
            fetched_at: None,
            domain: instance_domain.to_string(),
            banned: false,
            reason: None,
        }
    }
}
