use bayou_protocol::protocol::protocols::Protocols;

pub mod conn;
pub mod dbconn;
pub mod postgres;
#[cfg(test)]
pub mod tests;
pub mod utility;
pub mod newpost;

#[derive(Debug, Clone, Copy)]
pub struct Follower {
    pub uid: i64,
    pub is_local: bool,
    pub protocol: Protocols,
}

#[derive(Debug, Clone, Copy)]
pub struct Like {
    pub uid: i64,
    pub is_local: bool,
    pub obj_id: i64,
}
