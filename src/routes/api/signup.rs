use std::time::{self, SystemTime, UNIX_EPOCH};

use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Result,
};
use serde::{Deserialize, Serialize};

use crate::db::{
    pg_conn::PgConn,
    types::user::{SignupResult, SignupUser},
};

#[post("/signup")]
async fn signup(
    state: Data<crate::config::Config>,
    conn: Data<PgConn>,
    new_user: web::Json<SignupUser>,
) -> Result<HttpResponse> {
    let result = conn
        .try_signup_user(
            new_user.into_inner(),
            &state.instance_domain,
            state.allow_applications,
        )
        .await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok()
            .content_type("application/jrd+json; charset=utf-8")
            .body(
                serde_json::to_string(&SignupResult::Success)
                    .expect("failed to serialize signupresult"),
            )),
        Err(res) => Ok(HttpResponse::BadRequest()
            .content_type("application/jrd+json; charset=utf-8")
            .body(serde_json::to_string(&res).expect("failed to serialize signupresult"))),
    }
}
