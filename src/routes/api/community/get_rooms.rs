//! `get /api/bayou_v1/community/rooms/{comm_id}`
//!
//! get all rooms in a community, expects an auth token in the authorization header
//! - ok (200) should contain an array of [`crate::db::types::room::Room`]
//! should be present in the body
//! - unauthorized (401) included token is not valid

use crate::{db::pg_conn::PgConn, routes::api::utilities::auth_header::get_auth_header};
use actix_web::{
    get, web::{self, Data}, HttpRequest, HttpResponse, Result
};
use uuid::Uuid;

#[get("/rooms/{comm_id}")]
pub async fn get_rooms(
    conn: Data<PgConn>,
    req: HttpRequest,
    path: web::Path<Uuid>,
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
    let Ok(rooms) = conn
        .get_comm_rooms(path.into_inner(), token.uid)
        .await
    else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&rooms).expect("failed to serialize dbcommunity")))
}
