use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Result,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::{
        pg_conn::PgConn,
        types::{message::Messageinfo, room::RoomInfo},
    },
    routes::api::types::info_with_token::BearrerWithInfo,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewRoom {
    room_info: RoomInfo,
    #[serde(with = "uuid::serde::simple")]
    community: Uuid,
}

#[post("/new")]
pub async fn send_message(
    conn: Data<PgConn>,
    message: web::Json<BearrerWithInfo<Messageinfo>>,
) -> Result<HttpResponse> {
    if conn.validate_auth_token(&message.token).await.is_err() {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    }
    let Some(user) = conn.get_user_uid(&message.token.uid).await else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    };
    let message = match conn.send_message(&user, message.into_inner().info).await {
        Ok(message) => message,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized()
                .content_type("application/json; charset=utf-8")
                .body(""));
        }
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&message).expect("failed to serialize dbcommunity")))
}
