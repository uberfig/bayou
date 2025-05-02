//! `get /api/bayou_v1/community/rooms`
//!
//! get all rooms in a community, expects a [`uuid::Uuid`] wrapped in a [`crate::routes::api::types::info_with_token::BearrerWithInfo`]
//! - ok (200) should contain an array of [`crate::db::types::room::Room`]
//! should be present in the body
//! - unauthorized (401) included token is not valid, community not created

use crate::{db::pg_conn::PgConn, routes::api::types::info_with_token::BearrerWithInfo};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Result,
};
use uuid::Uuid;

#[get("/rooms")]
pub async fn get_rooms(
    conn: Data<PgConn>,
    community: web::Json<BearrerWithInfo<Uuid>>,
) -> Result<HttpResponse> {
    if conn.validate_auth_token(&community.token).await.is_err() {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    }
    let Ok(rooms) = conn
        .get_comm_rooms(community.info, community.token.uid)
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
