//! `get /api/bayou_v1/room/messages`
//!
//! get messages from a room, expects [`crate::db::types::message::Messageinfo`] with a token in the auth header
//! 
//! query params
//! - `room` required, room id
//! - `inclusive` optional, get messages including provided `older` or `newer` does nothing
//! if neither is present 
//! - `older` optional, message id, gets the messages older than the provided message
//! - `newer` optional, message id, gets the messages newer than the provided message
//! 
//! responses
//! - ok (200) list of [`crate::routes::api::types::api_message::ApiMessage`] in body
//! - unauthorized (401) included token is not valid to view given room

use crate::{
    db::pg_conn::PgConn,
    routes::api::types::{info_with_token::BearrerWithInfo, message_loader::MessagesLoader},
};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Result,
};

#[get("/messages")]
pub async fn get_messages(
    conn: Data<PgConn>,
    room: web::Json<BearrerWithInfo<MessagesLoader>>,
) -> Result<HttpResponse> {
    if conn.validate_auth_token(&room.token).await.is_err() {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    }
    println!("valid");
    let messages = match room.info.before {
        Some(before) => {
            let Ok(messages) = conn
                .get_room_messages_before(room.info.room, room.token.uid, before.time, before.post)
                .await
            else {
                return Ok(HttpResponse::Unauthorized()
                    .content_type("application/json; charset=utf-8")
                    .body(""));
            };
            messages
        }
        None => {
            println!("none");
            let Ok(messages) = conn.get_room_messages(room.info.room, room.token.uid).await else {
                return Ok(HttpResponse::Unauthorized()
                    .content_type("application/json; charset=utf-8")
                    .body(""));
            };
            messages
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&messages).expect("failed to serialize dbcommunity")))
}
