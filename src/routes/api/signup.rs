use std::time::{self, SystemTime, UNIX_EPOCH};

use actix_web::{post, web::{self, Data}, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::{cryptography::passwords::hash_password, db::{pg_conn::PgConn, types::user::{LocalUser, UserInfo}}};

#[derive(Serialize, Deserialize, Debug)]
pub struct Signup {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub token: Option<String>,
    pub application_message: Option<String>,
}

impl Signup {
    pub fn into_user(self, state: &Data<crate::config::Config>) -> UserInfo {
        // yes yes we are downcasting to an i64, if this is somehow still used
        // in 500 years then peeps can just use seconds instead of milis
        // or just upgrade to i128 or whatever they use in 500 years
        let curr_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_millis() as i64;

        UserInfo {
            domain: state.instance_domain.clone(),
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

#[post("/signup")]
async fn signup(state: Data<crate::config::Config>, conn: Data<PgConn>, new_user: web::Json<Signup>) -> Result<HttpResponse> {
    

    
    Ok(HttpResponse::Ok()
        .content_type("application/jrd+json; charset=utf-8")
        .body(""))
}
