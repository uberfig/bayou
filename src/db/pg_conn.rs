use crate::db::pg_sesh::Sesh;
use deadpool_postgres::Pool;
use uuid::Uuid;

use super::types::{instance::Instance, user::{DbUser, SignupResult, SignupUser}};

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

    pub async fn try_signup_user(&self, new_user: SignupUser, domain: &str, require_token: bool) -> Result<DbUser, SignupResult> {
        let mut client = self.db.get().await.expect("failed to get client");
        let transaction = client
            .transaction()
            .await
            .expect("failed to begin transaction");
        let sesh = Sesh::Transaction(transaction);
        if sesh.username_taken(&new_user.username, domain).await {
            return Err(SignupResult::UsernameTaken);
        }
        if require_token {
            let token = match &new_user.token {
                Some(token) => {
                    let Ok(token) = Uuid::parse_str(&token) else {
                        return Err(SignupResult::InvalidToken);
                    };
                    token
                },
                None => return Err(SignupResult::InvalidToken),
            };
            let Some(token) = sesh.get_signup_token(&token).await else {
                return Err(SignupResult::InvalidToken);
            };
            sesh.delete_signup_token(&token.id).await;
        }
        // we are now validated, create the user
        let user = sesh.create_user(new_user.into_user(domain)).await;
        sesh.commit().await;
        Ok(user)
    }
}
