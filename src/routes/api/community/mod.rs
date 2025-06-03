//! `/api/bayou_v1/community/...`
//! community specific methods such as creating and joining communities
pub mod create;
pub mod create_room;
pub mod get_joined;
pub mod get_members;
pub mod get_rooms;
pub(super) mod routes;
