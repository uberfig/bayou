use serde::{Deserialize, Serialize};

use crate::db::types::{tokens::auth_token::AuthToken, community::Communityinfo};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BearrerCommunityInfo {
    pub info: Communityinfo,
    pub token: AuthToken,
}
