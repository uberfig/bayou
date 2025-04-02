use std::time::{self, SystemTime, UNIX_EPOCH};

use actix_web::{post, web::{self, Data}, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::db::{pg_conn::PgConn, types::user::SignupUser};




#[post("/signup")]
async fn signup(state: Data<crate::config::Config>, conn: Data<PgConn>, new_user: web::Json<SignupUser>) -> Result<HttpResponse> {
    

    
    Ok(HttpResponse::Ok()
        .content_type("application/jrd+json; charset=utf-8")
        .body(""))
}
