use std::ops::DerefMut;

use crate::db::pg_sesh::Sesh;
use deadpool_postgres::Pool;
use uuid::Uuid;

use super::types::{
    auth_token::AuthToken,
    instance::Instance,
    registered_device::{DeviceInfo, RegisteredDevice},
    user::{DbUser, SignupResult, SignupUser},
};

#[derive(Clone, Debug)]
pub struct PgConn {
    pub db: Pool,
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

impl PgConn {
    /// apply database migrations
    pub async fn init(&self) -> Result<(), String> {
        let mut client = self.db.get().await.expect("failed to get client");
        let report = embedded::migrations::runner()
            .run_async(client.deref_mut().deref_mut())
            .await;
        match report {
            Ok(x) => {
                println!("migrations sucessful");
                if x.applied_migrations().is_empty() {
                    println!("no migrations applied")
                } else {
                    println!("applied migrations: ");
                    for migration in x.applied_migrations() {
                        match migration.applied_on() {
                            Some(x) => println!(" - {} applied {}", migration.name(), x),
                            None => println!(" - {} applied N/A", migration.name()),
                        }
                    }
                }
                Ok(())
            }
            Err(x) => {
                return Err(x.to_string());
            }
        }
    }
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

    pub async fn try_signup_user(
        &self,
        new_user: SignupUser,
        domain: &str,
        require_token: bool,
    ) -> Result<DbUser, SignupResult> {
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
                }
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

    pub async fn create_auth_token(&self, device: &Uuid, user: &Uuid) -> AuthToken {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.create_auth_token(device, user).await
    }

    pub async fn get_registered_device(&self, device_id: &Uuid) -> Option<RegisteredDevice> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.get_registered_device(device_id).await
    }

    pub async fn create_registered_device(&self, device: &DeviceInfo) -> RegisteredDevice {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.create_registered_device(device).await
    }
}
