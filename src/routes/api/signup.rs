//! `post /api/bayou_v1/signup`
//!
//! request with a username in the body and it will check if it has been taken
//! responses:
//! - ok (200) account successfully created and a [`crate::routes::api::types::signup_result::SignupResult::Success`]
//! should be present in the body
//! - bad request (400) account failed to be created, more information returned in the body as a
//! non success [`crate::routes::api::types::signup_result::SignupResult`]

use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Result,
};

use crate::{
    db::pg_conn::PgConn,
    routes::api::types::{signup_result::SignupResult, signup_user::SignupUser},
};

#[post("/signup")]
pub async fn signup(
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
            .content_type("application/json; charset=utf-8")
            .body(
                serde_json::to_string(&SignupResult::Success)
                    .expect("failed to serialize signupresult"),
            )),
        Err(res) => Ok(HttpResponse::BadRequest()
            .content_type("application/json; charset=utf-8")
            .body(serde_json::to_string(&res).expect("failed to serialize signupresult"))),
    }
}
