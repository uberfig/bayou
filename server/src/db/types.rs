use bayou_protocol::protocol::protocols::Protocols;
use url::Url;

#[derive(Debug, Clone)]
pub struct Follower {
    pub uuid: String,
    pub is_local: bool,
    pub protocol: Protocols,
}

#[derive(Debug, Clone)]
pub struct FollowerEndpoint {
    pub uuid: String,
    pub protocol: Protocols,
    pub inbox: Url,
}

#[derive(Debug, Clone, Copy)]
pub struct Like {
    pub uid: i64,
    pub is_local: bool,
    pub obj_id: i64,
}
