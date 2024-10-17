use bayou_protocol::protocol::protocols::Protocols;

pub mod conn;
pub mod postgres;
#[cfg(test)]
pub mod tests;
pub mod utility;

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
