use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SignupResult {
    UsernameTaken,
    InvalidToken,
    Success,
    InvalidUsername,
}
