use crate::db::{pg_conn::PgConn, types::tokens::auth_token::AuthToken};

use super::server::ChatServerHandle;

use std::time::{Duration, Instant};

use actix_ws::{AggregatedMessage, CloseReason};
use futures_util::StreamExt as _;
use tokio::{
    sync::mpsc,
    time::{interval, Interval},
};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

async fn await_token(
    session: &mut actix_ws::Session,
    msg_stream: &mut actix_ws::AggregatedMessageStream,
    last_heartbeat: &mut Instant,
    interval: &mut Interval,
) -> Result<AuthToken, Option<CloseReason>> {
    let pre_auth = loop {
        tokio::select! {
            Some(Ok(msg)) = msg_stream.next() => {
                match msg {
                    AggregatedMessage::Ping(bytes)=>{*last_heartbeat=Instant::now();session.pong(&bytes).await.unwrap();}
                    AggregatedMessage::Pong(_)=>{*last_heartbeat=Instant::now();}
                    AggregatedMessage::Text(text)=>{
                        if let Ok(val) = serde_json::from_str(&text) {
                            break Ok(val)
                        }
                    },
                    AggregatedMessage::Binary(_)  => break Err(None),
                    AggregatedMessage::Close(reason) => break Err(reason),
                }
            }
            _ = interval.tick() => {
                if Instant::now().duration_since(*last_heartbeat) > CLIENT_TIMEOUT {
                    break Err(None);
                }
                let _ = session.ping(b"").await;
            }
            else => {
                break Err(None);
            }
        }
    };
    pre_auth
}

pub async fn ws_handler(
    chat_server: ChatServerHandle,
    mut session: actix_ws::Session,
    msg_stream: actix_ws::MessageStream,
    conn: PgConn,
) {
    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let mut msg_stream = msg_stream
        .max_frame_size(128 * 1024)
        .aggregate_continuations()
        .max_continuation_size(2 * 1024 * 1024);

    let token = match await_token(
        &mut session,
        &mut msg_stream,
        &mut last_heartbeat,
        &mut interval,
    )
    .await
    {
        Ok(token) => token,
        Err(_) => {
            let _ = session.close(None).await;
            return;
        }
    };
    if conn.validate_auth_token(&token).await.is_err() {
        let _ = session.close(None).await;
        return;
    }

    let (conn_tx, mut outbound_messages) = mpsc::unbounded_channel();
    // unwrap: chat server is not dropped before the HTTP server
    let conn_id = chat_server.connect(conn_tx, token.uid).await;

    let close_reason = loop {
        tokio::select! {
            Some(Ok(msg)) = msg_stream.next() => {

                match msg {
                    AggregatedMessage::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        session.pong(&bytes).await.unwrap();
                    }

                    AggregatedMessage::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    // currently not used but will be in the
                    // future for things such as status information
                    // and any "live" communication
                    // may also be used for "subscribing" to
                    // a room
                    AggregatedMessage::Text(_text) => {}

                    // not allowed
                    AggregatedMessage::Binary(_bin) => break None,
                    AggregatedMessage::Close(reason) => break reason,
                }
            }

            Some(chat_msg) = outbound_messages.recv() => {
                 session.text(chat_msg).await.unwrap();
            }

            _ = interval.tick() => {
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    break None;
                }
                let _ = session.ping(b"").await;
            }

            else => {
                break None;
            }
        }
    };

    chat_server.disconnect(conn_id, token.uid);

    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;
}
