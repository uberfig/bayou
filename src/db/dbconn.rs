use std::ops::Deref;

use bayou_protocol::protocol::versia_protocol::{requests::Signer, verify::VersiaVerificationCache};

use super::{conn::Conn, postgres::pg_conn::PgConn};

#[derive(Clone, Debug)]
pub enum DbConn {
    Uncached(UncachedConn),
    Cached(CachedConn),
    Postgres(PgConn),
}

impl Deref for DbConn {
    type Target = dyn Conn + Sync;
    fn deref(&self) -> &Self::Target {
        match self {
            DbConn::Postgres(pg_conn) => pg_conn,
            DbConn::Uncached(uncached_conn) => &**uncached_conn,
            DbConn::Cached(cached_conn) => &**cached_conn,
        }
    }
}
impl VersiaVerificationCache for DbConn {
    async fn get_key(&self, signed_by: &Signer) -> Option<bayou_protocol::cryptography::openssl::OpenSSLPublic> {
        Box::pin(self.get_key(signed_by)).await
    }
}

#[derive(Clone, Debug)]
pub enum UncachedConn {
    Postgres(PgConn),
}

impl Deref for UncachedConn {
    type Target = dyn Conn + Sync;
    fn deref(&self) -> &Self::Target {
        match self {
            UncachedConn::Postgres(pg_conn) => pg_conn,
        }
    }
}


/// TODO create a moka type that impls conn and intercepts operations that 
/// can be cached. Moka will be a struct that holds an uncachedConn
#[derive(Clone, Debug)]
pub enum CachedConn {
    Moka(PgConn),
}

impl Deref for CachedConn {
    type Target = dyn Conn + Sync;
    fn deref(&self) -> &Self::Target {
        match self {
            CachedConn::Moka(pg_conn) => pg_conn,
        }
    }
}


