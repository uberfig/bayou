use std::ops::DerefMut;

use crate::db::{pg_sesh::Sesh, types::room::Room};
use deadpool_postgres::Pool;
use uuid::Uuid;

use super::{
    curr_time::get_current_time,
    types::{
        comm::{
            community::{Communityinfo, DbCommunity},
            community_membership::CommMembership,
        },
        instance::Instance,
        registered_device::{DeviceInfo, RegisteredDevice},
        room::RoomInfo,
        tokens::auth_token::{AuthToken, DBAuthToken},
        user::{DbUser, SignupResult, SignupUser},
    },
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
    pub async fn get_user_uid(&self, uid: &Uuid) -> Option<DbUser> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.get_user_uuid(uid).await
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

    pub async fn create_auth_token(&self, device: &Uuid, user: &Uuid) -> DBAuthToken {
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

    pub async fn validate_auth_token(&self, token: &AuthToken) -> Result<(), ()> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        let Some(retrived_token) = sesh.get_auth_token(&token.token).await else {
            return Err(());
        };
        if retrived_token.expiry < get_current_time() {
            sesh.delete_auth_token(&token.token).await;
            return Err(());
        }
        if retrived_token.required_token.device_id != token.device_id {
            // token is being used by a device it was not issued to
            return Err(());
        }
        if retrived_token.required_token.uid != token.uid {
            // we may wish to log this, attempting to use a token for a user it was
            // not issued to
            return Err(());
        }
        Ok(())
    }

    /// creates a new community and a general channel set as system
    /// channel and makes a membership for the creator
    pub async fn create_community(&self, info: Communityinfo, owner: &DbUser) -> DbCommunity {
        let mut client = self.db.get().await.expect("failed to get client");
        let transaction = client
            .transaction()
            .await
            .expect("failed to begin transaction");
        let sesh = Sesh::Transaction(transaction);
        let id = Uuid::now_v7();
        let community = DbCommunity {
            id,
            external_id: id,
            domain: owner.info.domain.clone(),
            info,
            created: get_current_time(),
            owner: owner.id,
        };
        let community = sesh.create_community(community).await;
        let room_id = Uuid::now_v7();
        let room = Room {
            id: room_id,
            external_id: room_id,
            domain: owner.info.domain.clone(),
            community: Some(id),
            system_channel: true,
            created: get_current_time(),
            is_dm: false,
            user_a: None,
            user_b: None,
            info: RoomInfo {
                name: "general".to_string(),
                description: None,
                custom_emoji: None,
                category: None,
                display_order: 0,
            },
            known_complete: true,
        };
        let _room = sesh.create_room(room).await;
        let _membership = sesh
            .create_comm_membership(CommMembership {
                com_id: community.id,
                uid: owner.id,
                joined: get_current_time(),
            })
            .await;
        sesh.commit().await;

        community
    }
    pub async fn get_community(&self, community: Uuid) -> Option<DbCommunity> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.get_community(&community).await
    }

    pub async fn create_comm_room(
        &self,
        community: &DbCommunity,
        user: Uuid,
        info: RoomInfo,
    ) -> Result<Room, ()> {
        // todo create role system and more fine grained permissions
        if community.owner != user {
            return Err(());
        }
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        let room_id = Uuid::now_v7();
        let room = Room {
            id: room_id,
            external_id: room_id,
            domain: community.domain.clone(),
            community: Some(community.id),
            system_channel: false,
            created: get_current_time(),
            is_dm: false,
            user_a: None,
            user_b: None,
            info,
            known_complete: true,
        };
        Ok(sesh.create_room(room).await)
    }
}
