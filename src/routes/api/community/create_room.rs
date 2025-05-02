use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Result,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::{pg_conn::PgConn, types::room::RoomInfo},
    routes::api::types::info_with_token::BearrerWithInfo,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NewRoom {
    room_info: RoomInfo,
    #[serde(with = "uuid::serde::simple")]
    community: Uuid,
}

#[post("/create_room")]
pub async fn create_room(
    conn: Data<PgConn>,
    new_room: web::Json<BearrerWithInfo<NewRoom>>,
) -> Result<HttpResponse> {
    if conn.validate_auth_token(&new_room.token).await.is_err() {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    }
    let Some(user) = conn.get_user_uid(&new_room.token.uid).await else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    };
    let Some(community) = conn.get_community(new_room.info.community).await else {
        return Ok(HttpResponse::NotFound()
            .content_type("application/json; charset=utf-8")
            .body(""));
    };
    let Ok(room) = conn
        .create_comm_room(&community, user.id, new_room.into_inner().info.room_info)
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
