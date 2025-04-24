use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AuthErr {
    InvalidToken,
    NotAuthorized,
    ExpiredToken,
}
