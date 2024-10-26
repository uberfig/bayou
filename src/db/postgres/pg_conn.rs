use deadpool_postgres::Pool;

use crate::db::conn::Conn;

use super::init;

#[derive(Clone, Debug)]
pub struct PgConn {
    pub db: Pool,
}

#[allow(unused_variables)]
impl Conn for PgConn {
    async fn init(&self) -> Result<(), String> {
        init::init(self).await
    }
}
