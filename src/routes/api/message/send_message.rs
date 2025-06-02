//! `post /api/bayou_v1/message/new`
//!
//! send a new message, expects [`crate::db::types::message::Messageinfo`] with a token in the auth header
//! - ok (200) message successfully sent
//! - unauthorized (401) included token is not valid or not allowed to send to given room, message not sent
//! - bad request (400) message is empty

use actix_web::{
    post, web::{self, Data}, HttpRequest, HttpResponse, Result
};
use serde::{Deserialize, Serialize};
use tokio::task::spawn_local;
use uuid::Uuid;

use crate::{
    db::{
        pg_conn::PgConn,
        types::{message::{DbMessage, Messageinfo}, room::RoomInfo},
    }, live_server::{server::{ChatServerHandle, MessageTarget}, socket_msg::SocketMsg}, routes::api::utilities::auth_header::get_auth_header
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewRoom {
    room_info: RoomInfo,
    #[serde(with = "uuid::serde::simple")]
    community: Uuid,
}

/// todo create get room members function in database
/// to support dms and fine grained control in communities
pub async fn message_notifyer(
    chat_server: web::Data<ChatServerHandle>,
    conn: Data<PgConn>,
    message: DbMessage,
) {
    let Some(room) = conn.get_room(message.info.room).await else {
        return ;
    };
    let Some(message) = conn.get_api_message(message.id).await else {
        return ;
    };
    let members = match room.community {
        Some(community) => {
            conn.get_comm_members(community).await
        },
        None => todo!(),
    };
    let members: Vec<Uuid> = members.into_iter().map(|x| x.id).collect();
    
    chat_server.send_message(SocketMsg::NewMessage(message), MessageTarget::List(members)).await;
}

#[post("/new")]
pub async fn send_message(
    req: HttpRequest,
    conn: Data<PgConn>,
    message: web::Json<Messageinfo>,
    chat_server: web::Data<ChatServerHandle>,
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
    let mut message = message.into_inner();
    message.content = message.content.trim().to_string();
    if message.content.is_empty() {
        return Ok(HttpResponse::BadRequest()
            .content_type("application/json; charset=utf-8")
            .body(""));
    }
    let message = match conn.send_message(&user, message).await {
        Ok(message) => message,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized()
                .content_type("application/json; charset=utf-8")
                .body(""));
        }
    };
    spawn_local(message_notifyer(chat_server, conn, message));
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string("").expect("failed to serialize dbcommunity")))
}
