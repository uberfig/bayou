//! `post /api/bayou_v1/community/create_room`
//!
//! get all rooms in a community, expects an auth token in the authorization header
//! and a body with a [`NewRoom`]
//! - ok (200) should contain a json [`crate::db::types::room::Room`]
//! in the body
//! - unauthorized (401) included token is not valid or authorized to create 
//! rooms in the given community

use actix_web::{
    post, web::{self, Data}, HttpRequest, HttpResponse, Result
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::{pg_conn::PgConn, types::room::RoomInfo}, routes::api::utilities::auth_header::get_auth_header};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NewRoom {
    room_info: RoomInfo,
    community: Uuid,
}

#[post("/create_room")]
pub async fn create_room(
    req: HttpRequest,
    conn: Data<PgConn>,
    new_room: web::Json<NewRoom>,
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
    let Some(user) = conn.get_user_uid(&token.uid).await else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    };
    let Some(community) = conn.get_community(new_room.community).await else {
        return Ok(HttpResponse::NotFound()
            .content_type("application/json; charset=utf-8")
            .body(""));
    };
    let Ok(room) = conn
        .create_comm_room(&community, user.id, new_room.into_inner().room_info)
        .await
    else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&room).expect("failed to serialize dbcommunity")))
}
