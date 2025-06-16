use std::ops::DerefMut;

use crate::{
    db::{pg_sesh::Sesh, types::room::Room},
    routes::api::types::{
        api_community::ApiCommunity, api_message::ApiMessage, api_user::ApiUser,
        signup_result::SignupResult, signup_user::SignupUser,
    },
};
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
        message::{DbMessage, Messageinfo},
        registered_device::{DeviceInfo, RegisteredDevice},
        room::RoomInfo,
        tokens::auth_token::{AuthToken, DBAuthToken},
        user::DbUser,
    },
};

pub const MAX_PAGENATION: i64 = 40;

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
        mut new_user: SignupUser,
        domain: &str,
        require_token: bool,
    ) -> Result<DbUser, SignupResult> {
        if !new_user
            .username
            .chars()
            .all(|x: char| char::is_ascii_alphanumeric(&x) || x.eq(&'_'))
        {
            return Err(SignupResult::InvalidUsername);
        }
        new_user.username = new_user.username.to_ascii_lowercase();
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
                Some(token) => token,
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
            domain: owner.domain.clone(),
            info,
            created: get_current_time(),
            owner: owner.id,
        };
        let community = sesh.create_community(community).await;
        let room_id = Uuid::now_v7();
        let room = Room {
            id: room_id,
            external_id: room_id,
            domain: owner.domain.clone(),
            community: Some(id),
            system_channel: true,
            created: get_current_time(),
            is_dm: false,
            user_a: None,
            user_b: None,
            info: RoomInfo {
                name: "general".to_string(),
                description: None,
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
    pub async fn get_comm_membership(&self, com_id: Uuid, uid: Uuid) -> Option<CommMembership> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.get_comm_membership(&com_id, &uid).await
    }
    /// get all rooms from a community if it exists and the user is in the community
    /// - caching here might be useful
    pub async fn get_comm_rooms(&self, com_id: Uuid, uid: Uuid) -> Result<Vec<Room>, ()> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        let Some(_membership) = sesh.get_comm_membership(&com_id, &uid).await else {
            return Err(());
        };
        Ok(sesh.get_all_comm_rooms(&com_id).await)
    }
    /// get all communities a user is a member of
    pub async fn get_all_joined(&self, uid: Uuid) -> Vec<ApiCommunity> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.get_all_user_comms(&uid)
            .await
            .into_iter()
            .map(|x| DbCommunity::from(x).into())
            .collect()
    }
    /// get all members from a community if it exists and the user is in the community
    /// - caching here might be useful
    pub async fn user_get_comm_members(&self, com_id: Uuid, uid: Uuid) -> Result<Vec<ApiUser>, ()> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        let Some(_membership) = sesh.get_comm_membership(&com_id, &uid).await else {
            return Err(());
        };
        Ok(sesh
            .get_all_comm_users(&com_id)
            .await
            .into_iter()
            .map(|x| x.into())
            .collect())
    }
    pub async fn get_comm_members(&self, com_id: Uuid) -> Vec<ApiUser> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.get_all_comm_users(&com_id)
            .await
            .into_iter()
            .map(|x| x.into())
            .collect()
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
    pub async fn get_room(&self, room: Uuid) -> Option<Room> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.get_room(&room).await
    }
    /// attempt to send message to given room
    /// returns err if room does not exist or not authorized to post in room
    /// todo: add fine grained channel controls
    /// todo: add more descriptive errors and use them in the api
    pub async fn send_message(&self, user: &DbUser, message: Messageinfo) -> Result<DbMessage, ()> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        let Some(room) = sesh.get_room(&message.room).await else {
            return Err(());
        };
        // ensure the user is allowed to post in the given room
        match room.community {
            // being posted to a room in a community
            Some(com_id) => {
                let Some(_) = sesh.get_comm_membership(&com_id, &user.id).await else {
                    return Err(());
                };
            }
            // being posted to a dm or group chat
            None => return Err(()),
        };

        // ensure that the message is replying to a message that exists and in this channel
        if let Some(reply) = message.in_reply_to {
            let Some(reply) = sesh.get_message(&reply).await else {
                return Err(());
            };
            if reply.info.room != message.room {
                return Err(());
            }
        }

        let id = Uuid::now_v7();
        let message = DbMessage {
            id,
            external_id: id,
            domain: user.domain.clone(),
            user: user.id,
            published: get_current_time(),
            edited: None,
            fetched_at: None,
            info: message,
        };
        Ok(sesh.create_message(message).await)
    }

    pub async fn get_api_message(&self, m_id: Uuid) -> Option<ApiMessage> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.get_api_message(&m_id).await
    }

    pub async fn get_room_messages(&self, room_id: Uuid, uid: Uuid) -> Result<Vec<ApiMessage>, ()> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        let Some(room) = sesh.get_room(&room_id).await else {
            return Err(());
        };
        match room.community {
            Some(com_id) => {
                let Some(_membership) = sesh.get_comm_membership(&com_id, &uid).await else {
                    return Err(());
                };
            }
            None => todo!(),
        };
        Ok(sesh.get_room_messages(&room_id, MAX_PAGENATION).await)
    }
    pub async fn get_room_messages_in_relation(
        &self,
        room_id: Uuid,
        uid: Uuid,
        post: Uuid,
        inclusive: bool,
        // get posts before or after the post, older or newer
        before: bool,
    ) -> Result<Vec<ApiMessage>, ()> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        let Some(room) = sesh.get_room(&room_id).await else {
            return Err(());
        };
        match room.community {
            Some(com_id) => {
                let Some(_membership) = sesh.get_comm_membership(&com_id, &uid).await else {
                    return Err(());
                };
            }
            None => todo!(),
        };
        match before {
            true => Ok(sesh
                .get_room_messages_before(room_id, MAX_PAGENATION, post, inclusive)
                .await),
            false => Ok(sesh
                .get_room_messages_after(room_id, MAX_PAGENATION, post, inclusive)
                .await),
        }
    }

    pub async fn username_taken(&self, username: &str, domain: &str) -> bool {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.username_taken(username, domain).await
    }
}
