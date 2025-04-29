use serde::{Deserialize, Serialize};

use crate::db::types::tokens::auth_token::AuthToken;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BearrerWithInfo<T> {
    pub info: T,
    pub token: AuthToken,
}
