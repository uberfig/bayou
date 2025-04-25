use serde::{Deserialize, Serialize};

use crate::db::types::{comm::community::Communityinfo, tokens::auth_token::AuthToken};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BearrerCommunityInfo {
    pub info: Communityinfo,
    pub token: AuthToken,
}
