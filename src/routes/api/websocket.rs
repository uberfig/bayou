//! `websocket /api/bayou_v1/ws`
//!
//! expects an [`crate::db::types::tokens::auth_token::AuthToken`] sent
//! through the channel upon connection, will send live messages to the
//! client to be used for rendering the message log

use actix_web::{
    post,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};
use tokio::task::spawn_local;

use crate::{
    db::pg_conn::PgConn,
    live_server::{server::ChatServerHandle, socket_handler::ws_handler},
};

#[post("/ws")]
pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    chat_server: web::Data<ChatServerHandle>,
    conn: Data<PgConn>,
) -> Result<HttpResponse> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    spawn_local(ws_handler(
        (**chat_server).clone(),
        session,
        msg_stream,
        (**conn).clone(),
    ));

    Ok(res)
}
