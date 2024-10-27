use crate::db::conn::DbErr;
use bayou_protocol::cryptography::key::Algorithms;
use bayou_protocol::protocol::versia_protocol::{
    requests::Signer, verify::VersiaVerificationCache,
};
use bayou_protocol::{
    cryptography::openssl::OpenSSLPublic,
    types::{
        activitystream_objects::{actors::Actor, new_post::NewPost, postable::ApPostable},
        versia_types::{
            entities::{instance_metadata::InstanceMetadata, user::User},
            postable::VersiaPostable,
        },
    },
};
use enum_dispatch::enum_dispatch;
use url::Url;

use super::{
    conn::EntityOrigin,
    types::{Follower, FollowerEndpoint},
    utility::{instance_actor::InstanceActor, new_actor::NewLocal, protocols::Protocol},
};

use super::{conn::Conn, postgres::pg_conn::PgConn};

#[derive(Clone, Debug)]
#[enum_dispatch]
pub enum DbConn {
    Uncached(UncachedConn),
    Cached(CachedConn),
    Postgres(PgConn),
}

impl VersiaVerificationCache for DbConn {
    async fn verify_get_key(&self, signed_by: &Signer) -> Option<OpenSSLPublic> {
        Box::pin(self.get_key(signed_by)).await
    }
}

#[derive(Clone, Debug)]
#[enum_dispatch]
pub enum UncachedConn {
    Postgres(PgConn),
}

/// TODO create a moka type that impls conn and intercepts operations that
/// can be cached. Moka will be a struct that holds an uncachedConn
#[derive(Clone, Debug)]
#[enum_dispatch]
pub enum CachedConn {
    Moka(PgConn),
}
