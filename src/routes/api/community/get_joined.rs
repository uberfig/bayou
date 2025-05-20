//! `get /api/bayou_v1/community/joined`
//!
//! get all of a user's communities, expects an [`crate::db::types::tokens::auth_token::AuthToken`]
//! - ok (200) should contain an array of [`crate::routes::api::types::api_community::ApiCommunity`]
//! present in the body
//! - unauthorized (401) included token is not valid

use crate::{db::pg_conn::PgConn, routes::api::utilities::auth_header::get_auth_header};
use actix_web::{
    get, web::Data, HttpRequest, HttpResponse, Result
};

#[get("/joined")]
pub async fn get_joined(
    conn: Data<PgConn>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let Some(token) = get_auth_header(&req) else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body("invalid or missing auth header"));
    };
    if conn.validate_auth_token(&token).await.is_err() {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    }
    let joined = conn.get_all_joined(token.uid).await;
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&joined).expect("failed to serialize dbcommunity")))
}
