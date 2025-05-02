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
