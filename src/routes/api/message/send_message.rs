use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Result,
};
use serde::{Deserialize, Serialize};
use tokio::task::spawn_local;
use uuid::Uuid;

use crate::{
    db::{
        pg_conn::PgConn,
        types::{message::{DbMessage, Messageinfo}, room::RoomInfo},
    }, live_server::{server::{ChatServerHandle, MessageTarget}, socket_msg::SocketMsg}, routes::api::types::info_with_token::BearrerWithInfo
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
    conn: Data<PgConn>,
    message: web::Json<BearrerWithInfo<Messageinfo>>,
    chat_server: web::Data<ChatServerHandle>,
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
    spawn_local(message_notifyer(chat_server, conn, message));
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string("").expect("failed to serialize dbcommunity")))
}
