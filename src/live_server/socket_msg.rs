use serde::{Deserialize, Serialize};

use crate::routes::api::types::api_message::ApiMessage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SocketMsg {
    NewMessage(ApiMessage),
    SystemMessage(String),
}
