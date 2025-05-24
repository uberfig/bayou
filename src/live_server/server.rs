use std::{collections::{HashMap, HashSet}, io};

use uuid::Uuid;
use tokio::sync::{mpsc, oneshot};

use super::socket_msg::SocketMsg;

pub type ConnId = Uuid;
pub type RoomId = Uuid;
pub type UserId = Uuid;

/// serialized message to be sent to the client
type Msg = String;

impl From<SocketMsg> for Msg {
    fn from(value: SocketMsg) -> Self {
        serde_json::to_string(&value).expect("failed to serialize")
    }
}

/// we take a list of message targets so that then 
/// its not our job to handle that, the correct recipients 
/// should be grabbed from the database and permissions 
/// should all be handled before the messages come here
/// since the message server is just effectively 
/// single threadded for now
pub enum MessageTarget {
    All,
    List(Vec<UserId>),
}

pub enum Command {
    Connect {
        conn_sender: mpsc::UnboundedSender<Msg>,
        user: UserId,
        /// used to pass the conn id back to the websocket
        /// handler once we have registered them, this is needed
        /// so that they may send a disconnect when they are 
        /// disconnected
        response_handle: oneshot::Sender<ConnId>,
    },
    Disconnect {
        user: UserId,
        conn: ConnId,
    },
    BroadcastMessage {
        msg: SocketMsg,
        recipients: MessageTarget,
        /// used to notify upon completion 
        response_handle: oneshot::Sender<()>,
    }
    // todo create a join room command for room status information 
    // (users online and their statusses, currently typing etc)
}

/// this is largely based off https://github.com/actix/examples/blob/master/websockets/chat-actorless/src/server.rs
/// down the road we're going to need to figure out how to better parallelize it
pub struct ChatServer {
    /// all active websocket connections
    sessions: HashMap<ConnId, mpsc::UnboundedSender<Msg>>,
    /// used to get sessions spawned by the user, if none exist the user
    /// should be removed from this map
    user_sessions: HashMap<UserId, HashSet<ConnId>>,
    cmd_reciever: mpsc::UnboundedReceiver<Command>,
}

impl ChatServer {
    pub fn new() -> (Self, ChatServerHandle) {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let new = Self {
            sessions: HashMap::new(),
            user_sessions: HashMap::new(),
            cmd_reciever: cmd_rx,
        };
        (new, ChatServerHandle{cmd_tx})
    }
    async fn handle_message(&self, recipients: MessageTarget, msg: impl Into<Msg>) {
        let msg: Msg = msg.into();
        match recipients {
            MessageTarget::All => {
                for (_user, sender) in &self.sessions {
                    let _ = sender.send(msg.clone());
                }
            },
            MessageTarget::List(uuids) => {
                for user in uuids {
                    if let Some(conns) = self.user_sessions.get(&user) {
                        // user is online, send to all connections
                        for conn in conns {
                            if let Some(sender) = self.sessions.get(conn) {
                                let _ = sender.send(msg.clone());
                            }
                        }
                    }
                }
            },
        }
    }
    /// todo, pivot to using rwlocks for the sessions
    async fn connect(&mut self, conn_sender: mpsc::UnboundedSender<Msg>, user: ConnId) -> ConnId {
        let id: ConnId = Uuid::new_v4();
        self.sessions.insert(id, conn_sender);
        match self.user_sessions.get_mut(&user) {
            Some(sessions) => {
                sessions.insert(id);
            },
            None => {
                let mut set = HashSet::new();
                set.insert(id);
                self.user_sessions.insert(user, set);
            },
        }
        id
    }
    async fn disconnect(&mut self, user: UserId, conn: ConnId) {
        self.sessions.remove(&conn);
        match self.user_sessions.get_mut(&user) {
            Some(sessions) => {
                sessions.remove(&conn);
                if sessions.is_empty() {
                    self.user_sessions.remove(&user);
                }
            },
            // this shouldn't happen and should prob be logged if it has
            // as something has gone pretty wrong if so
            None => {},
        }
    }
    pub async fn run(mut self) -> io::Result<()> {
        while let Some(cmd) = self.cmd_reciever.recv().await {
            match cmd {
                Command::Connect {conn_sender, user, response_handle } => {
                    let conn_id = self.connect(conn_sender, user).await;
                    let _ = response_handle.send(conn_id);
                }

                Command::Disconnect { conn, user } => {
                    self.disconnect(user, conn).await;
                }

                Command::BroadcastMessage { msg, recipients, response_handle } => {
                    self.handle_message(recipients, msg).await;
                    let _ = response_handle.send(());
                }
            }
        }

        Ok(())
    }
}

/// Handle and command sender for chat server.
///
/// Reduces boilerplate of setting up response channels in WebSocket handlers.
/// taken from https://github.com/actix/examples/blob/master/websockets/chat-actorless/src/server.rs#L227
#[derive(Debug, Clone)]
pub struct ChatServerHandle {
    cmd_tx: mpsc::UnboundedSender<Command>,
}

impl ChatServerHandle {
    pub async fn connect(&self, conn_sender: mpsc::UnboundedSender<Msg>, user: UserId,) -> ConnId {
        let (res_tx, res_rx) = oneshot::channel();
        // unwraps used as the server should run until all chat server handles are dropped
        // and then nicely shutdown itself. the server should always be shutting down after
        // us here, never before
        self.cmd_tx
            .send(Command::Connect { conn_sender, user, response_handle: res_tx })
            .unwrap();
        res_rx.await.unwrap()
    }
    pub fn disconnect(&self, conn: ConnId, user: UserId) {
        self.cmd_tx.send(Command::Disconnect { user, conn }).unwrap();
    }
    pub async fn send_message(&self, msg: SocketMsg, recipients: MessageTarget,) {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx
            .send(Command::BroadcastMessage { msg, recipients, response_handle: res_tx })
            .unwrap();
        res_rx.await.unwrap()
    }
}