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
//! only one of `older` or `newer` should be passed, both is intentionally undefined
//! behavior as it is subject to change or being rejected
//!
//! responses
//! - ok (200) list of [`crate::routes::api::types::api_message::ApiMessage`] in body
//! - unauthorized (401) included token is not valid to view given room

use crate::{
    db::pg_conn::PgConn,
    routes::api::{types::api_message::ApiMessage, utilities::auth_header::get_auth_header},
};
use actix_web::{
    get,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct GetMessagesQuery {
    pub room: Uuid,
    pub inclusive: Option<bool>,
    pub older: Option<Uuid>,
    pub newer: Option<Uuid>,
}

fn return_result(messages: &Vec<ApiMessage>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(messages).expect("failed to serialize dbcommunity")))
}

#[get("/messages")]
pub async fn get_messages(
    conn: Data<PgConn>,
    info: web::Query<GetMessagesQuery>,
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

    if let Some(older) = info.older {
        let Ok(messages) = conn
            .get_room_messages_in_relation(
                info.room,
                token.uid,
                older,
                info.inclusive.unwrap_or(false),
                true,
            )
            .await
        else {
            return Ok(HttpResponse::Unauthorized()
                .content_type("application/json; charset=utf-8")
                .body(""));
        };
        return return_result(&messages);
    }

    if let Some(newer) = info.newer {
        let Ok(messages) = conn
            .get_room_messages_in_relation(
                info.room,
                token.uid,
                newer,
                info.inclusive.unwrap_or(false),
                false,
            )
            .await
        else {
            return Ok(HttpResponse::Unauthorized()
                .content_type("application/json; charset=utf-8")
                .body(""));
        };
        return return_result(&messages);
    }

    let Ok(messages) = conn.get_room_messages(info.room, token.uid).await else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    };
    return_result(&messages)
}
