use crate::db::pg_sesh::Sesh;
use deadpool_postgres::Pool;

use super::types::{instance::Instance, user::DbUser};

#[derive(Clone, Debug)]
pub struct PgConn {
    pub db: Pool,
}

impl PgConn {
    /// gets the main instance if exists or creates a new
    /// should be run on startup to ensure db is ready
    pub async fn get_or_init_main_instance(&self, domain: &str) -> Instance {
        let mut client = self.db.get().await.expect("failed to get client");
        let transaction = client
            .transaction()
            .await
            .expect("failed to begin transaction");
        let sesh = Sesh::Transaction(transaction);
        if let Some(instance) = sesh.get_instance(domain).await {
            return instance;
        }
        //init the instance
        let instance = sesh.create_instance(domain, true, false, None, true).await;
        sesh.commit().await;
        instance
    }

    pub async fn get_user(&self, username: &str, domain: &str) -> Option<DbUser> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.get_user(username, domain).await
    }
}
