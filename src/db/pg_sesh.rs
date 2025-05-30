use crate::routes::api::types::api_message::ApiMessage;

use super::{
    curr_time::{get_current_time, get_expiry},
    types::{
        comm::{community::DbCommunity, community_membership::CommMembership},
        instance::Instance,
        message::DbMessage,
        registered_device::{DeviceInfo, RegisteredDevice},
        room::Room,
        tokens::{auth_token::DBAuthToken, signup_token::SignupToken},
        user::DbUser,
    },
};
use deadpool_postgres::{Object, Transaction};
use tokio_postgres::{types::ToSql, Statement};
use uuid::Uuid;

pub enum Sesh<'a> {
    Client(Object),
    Transaction(Transaction<'a>),
}
impl Sesh<'_> {
    pub async fn commit(self) {
        if let Sesh::Transaction(transaction) = self {
            transaction.commit().await.expect("failed to commit")
        }
    }
    pub async fn query(
        &self,
        stmt: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> {
        let stmt = self.prepare(stmt).await;
        self.query_stmt(&stmt, params).await
    }
    pub async fn prepare(&self, stmt: &str) -> Statement {
        match self {
            Sesh::Client(object) => object.prepare(stmt).await.expect("failed to prepare query"),
            Sesh::Transaction(transaction) => transaction
                .prepare(stmt)
                .await
                .expect("failed to prepare query"),
        }
    }
    pub async fn query_stmt(
        &self,
        stmt: &Statement,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> {
        match self {
            Sesh::Client(object) => object.query(stmt, params).await,
            Sesh::Transaction(transaction) => transaction.query(stmt, params).await,
        }
    }
}

//---------------------------- users --------------------
#[allow(dead_code)]
impl Sesh<'_> {
    pub async fn get_user(&self, username: &str, domain: &str) -> Option<DbUser> {
        let result = self
            .query(DbUser::read_statement(), &[&username, &domain])
            .await
            .expect("failed to fetch user")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn get_user_uuid(&self, uid: &Uuid) -> Option<DbUser> {
        let result = self
            .query(DbUser::read_uid_statement(), &[&uid])
            .await
            .expect("failed to fetch user")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn create_user(&self, new_user: DbUser) -> DbUser {
        let result = self
            .query(
                DbUser::create_statement(),
                &[
                    &new_user.id,
                    &new_user.domain,
                    &new_user.info.username,
                    &new_user.info.display_name,
                    &new_user.info.summary,
                    &new_user.banned,
                    &new_user.reason,
                    &new_user.fetched_at,
                    &new_user.local_info.is_some(),
                    &new_user.local_info.as_ref().map(|x| x.password.clone()),
                    &new_user.local_info.as_ref().map(|x| x.email.clone()),
                    &new_user.local_info.as_ref().map(|x| x.verified),
                    &new_user.local_info.as_ref().map(|x| x.is_admin),
                    &new_user.local_info.as_ref().map(|x| x.instance_mod),
                    &new_user
                        .local_info
                        .as_ref()
                        .map(|x| x.application_message.clone()),
                    &new_user.local_info.as_ref().map(|x| x.application_approved),
                    &new_user.info.created,
                ],
            )
            .await
            .expect("failed to insert user")
            .pop()
            .expect("inserting user returned nothing");
        result.into()
    }
    pub async fn update_user(&self, user: DbUser) -> DbUser {
        let result = self
            .query(
                DbUser::update_statement(),
                &[
                    &user.info.display_name,
                    &user.info.summary,
                    &user.local_info.as_ref().map(|x| x.instance_mod),
                    &user.banned,
                    &user.reason,
                    &user.fetched_at,
                    &user.local_info.is_some(),
                    &user.local_info.as_ref().map(|x| x.password.clone()),
                    &user.local_info.as_ref().map(|x| x.email.clone()),
                    &user.local_info.as_ref().map(|x| x.verified),
                    &user.local_info.as_ref().map(|x| x.is_admin),
                    &user.id,
                ],
            )
            .await
            .expect("failed to update user")
            .pop()
            .expect("updating user returned nothing");
        result.into()
    }
    pub async fn delete_user(&self, user: DbUser) {
        let _result = self
            .query(DbUser::delete_statement(), &[&user.id])
            .await
            .expect("failed to delete user");
    }
    #[allow(unused_variables)]
    pub async fn set_user_banned(&self, user: &DbUser, banned: bool, reason: Option<String>) {
        todo!()
    }
    /// cheaper query to use instead of getting a user just to discard the data
    pub async fn username_taken(&self, username: &str, domain: &str) -> bool {
        let stmt = r#"
            SELECT uid FROM users WHERE username = $1 AND domain = $2;
        "#;
        let result = self
            .query(stmt, &[&username, &domain])
            .await
            .expect("failed to fetch user")
            .pop();
        result.is_some()
    }
}

// --------------------------------- instance --------------------
#[allow(dead_code)]
impl Sesh<'_> {
    pub async fn create_instance(
        &self,
        domain: &str,
        is_authoratative: bool,
        banned: bool,
        reason: Option<String>,
        allowlist: bool,
    ) -> Instance {
        let result = self
            .query(
                Instance::create_statement(),
                &[&domain, &is_authoratative, &banned, &reason, &allowlist],
            )
            .await
            .expect("failed to create instance")
            .pop()
            .expect("creating instance returned nothing");
        result.into()
    }
    pub async fn get_instance(&self, domain: &str) -> Option<Instance> {
        let stmt = r#"
            SELECT * FROM instances WHERE domain = $1;
        "#;
        let result = self
            .query(stmt, &[&domain])
            .await
            .expect("failed to fetch instance")
            .pop();
        result.map(|x| x.into())
    }
    /// ban an istance without severing any connections or deleting data, will pause any future following
    /// and any incoming and outgoing traffic to this instance will stop
    ///
    /// to delete and ban, create a transaction and use [`Sesh::delete_instance`] and then [`Sesh::create_instance`]
    /// with banned set to true
    pub async fn update_instance(&self, instance: Instance) -> Instance {
        let result = self
            .query(
                Instance::update_statement(),
                &[
                    &instance.is_authoratative,
                    &instance.blocked,
                    &instance.reason,
                    &instance.allowlisted,
                    &instance.domain,
                ],
            )
            .await
            .expect("failed to update instance")
            .pop()
            .expect("updating instance returned nothing");
        result.into()
    }
    pub async fn delete_instance(&self, instance: Instance) {
        let _result = self
            .query(Instance::delete_statement(), &[&instance.domain])
            .await
            .expect("failed to delete instance");
    }
}

// ------------------------- signup token -----------------------------
#[allow(dead_code)]
impl Sesh<'_> {
    pub async fn create_signup_token(&self, creator: &DbUser, expiry: i64) -> SignupToken {
        let id = Uuid::new_v4();
        let result = self
            .query(
                SignupToken::create_statement(),
                &[&id, &creator.id, &expiry],
            )
            .await
            .expect("failed to create signup token")
            .pop()
            .expect("creating signup token returned nothing");
        result.into()
    }
    pub async fn get_signup_token(&self, token_id: &Uuid) -> Option<SignupToken> {
        let result = self
            .query(SignupToken::read_statement(), &[token_id])
            .await
            .expect("failed to fetch signup token")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn delete_signup_token(&self, token_id: &Uuid) {
        let _result = self
            .query(SignupToken::delete_statement(), &[token_id])
            .await
            .expect("failed to delete signup token");
    }
}

// ------------------------- registered device -----------------------------
#[allow(dead_code)]
impl Sesh<'_> {
    pub async fn create_registered_device(&self, device: &DeviceInfo) -> RegisteredDevice {
        let id = Uuid::now_v7();
        let result = self
            .query(
                RegisteredDevice::create_statement(),
                &[
                    &id,
                    &device.device_name,
                    &device.software,
                    &device.webpage,
                    &device.redirect_url,
                    &get_current_time(),
                ],
            )
            .await
            .expect("failed to create registered device")
            .pop()
            .expect("creating registered device returned nothing");
        result.into()
    }
    pub async fn get_registered_device(&self, device_id: &Uuid) -> Option<RegisteredDevice> {
        let result = self
            .query(RegisteredDevice::read_statement(), &[device_id])
            .await
            .expect("failed to fetch registered device")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn delete_registered_device(&self, device_id: &Uuid) {
        let _result = self
            .query(RegisteredDevice::delete_statement(), &[device_id])
            .await
            .expect("failed to delete registered device");
    }
}

// ------------------------- auth tokens -----------------------------
#[allow(dead_code)]
impl Sesh<'_> {
    pub async fn create_auth_token(&self, device: &Uuid, user: &Uuid) -> DBAuthToken {
        let id = Uuid::new_v4();
        let expiry = get_expiry(60);
        let result = self
            .query(
                DBAuthToken::create_statement(),
                &[&id, &device, &user, &expiry],
            )
            .await
            .expect("failed to create auth token")
            .pop()
            .expect("creating auth token returned nothing");
        result.into()
    }
    pub async fn get_auth_token(&self, token_id: &Uuid) -> Option<DBAuthToken> {
        let result = self
            .query(DBAuthToken::read_statement(), &[token_id])
            .await
            .expect("failed to fetch auth token")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn delete_auth_token(&self, token_id: &Uuid) {
        let _result = self
            .query(DBAuthToken::delete_statement(), &[token_id])
            .await
            .expect("failed to delete registered device");
    }
}

// ------------------------- community -----------------------------
#[allow(dead_code)]
impl Sesh<'_> {
    pub async fn create_community(&self, community: DbCommunity) -> DbCommunity {
        let result = self
            .query(
                DbCommunity::create_statement(),
                &[
                    &community.id,
                    &community.external_id,
                    &community.domain,
                    &community.info.name,
                    &community.info.description,
                    &community.created,
                    &community.owner,
                ],
            )
            .await
            .expect("failed to create community")
            .pop()
            .expect("creating community returned nothing");
        result.into()
    }
    pub async fn get_community(&self, com_id: &Uuid) -> Option<DbCommunity> {
        let result = self
            .query(DbCommunity::read_statement(), &[com_id])
            .await
            .expect("failed to fetch community")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn update_community(&self, communnity: DbCommunity) -> DbCommunity {
        let result = self
            .query(
                DbCommunity::update_statement(),
                &[
                    &communnity.external_id,
                    &communnity.domain,
                    &communnity.owner,
                    &communnity.info.name,
                    &communnity.info.description,
                    &communnity.id,
                ],
            )
            .await
            .expect("failed to update community")
            .pop()
            .expect("updating community returned nothing");
        result.into()
    }
    pub async fn delete_community(&self, com_id: &Uuid) {
        let _result = self
            .query(DbCommunity::delete_statement(), &[com_id])
            .await
            .expect("failed to delete community");
    }
}

// ------------------------- community membership -----------------------------
#[allow(dead_code)]
impl Sesh<'_> {
    pub async fn create_comm_membership(&self, membership: CommMembership) -> CommMembership {
        let result = self
            .query(
                CommMembership::create_statement(),
                &[&membership.com_id, &membership.uid, &membership.joined],
            )
            .await
            .expect("failed to create community membership")
            .pop()
            .expect("creating community membership returned nothing");
        result.into()
    }
    pub async fn get_comm_membership(&self, com_id: &Uuid, uid: &Uuid) -> Option<CommMembership> {
        let result = self
            .query(CommMembership::read_statement(), &[com_id, uid])
            .await
            .expect("failed to fetch community membership")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn delete_comm_membership(&self, com_id: &Uuid, uid: &Uuid) {
        let _result = self
            .query(CommMembership::delete_statement(), &[com_id, uid])
            .await
            .expect("failed to delete community membership");
    }
    /// gets all user membership joined on communities
    pub async fn get_all_user_comms(&self, uid: &Uuid) -> Vec<tokio_postgres::Row> {
        let result = self
            .query(CommMembership::get_all_user_comms(), &[uid])
            .await
            .expect("failed to fetch user communities");
        result
    }
    pub async fn get_all_comm_members(&self, com_id: &Uuid) -> Vec<CommMembership> {
        let result = self
            .query(CommMembership::get_all_comm_members(), &[com_id])
            .await
            .expect("failed to fetch community members");
        result.into_iter().map(|x| x.into()).collect()
    }
    pub async fn get_all_comm_users(&self, com_id: &Uuid) -> Vec<DbUser> {
        let result = self
            .query(CommMembership::get_all_comm_users(), &[com_id])
            .await
            .expect("failed to fetch community users");
        result.into_iter().map(|x| x.into()).collect()
    }
}

// ------------------------- room -----------------------------
#[allow(dead_code)]
impl Sesh<'_> {
    pub async fn create_room(&self, room: Room) -> Room {
        // let id = Uuid::now_v7();
        let result = self
            .query(
                Room::create_statement(),
                &[
                    &room.id,
                    &room.external_id,
                    &room.domain,
                    &room.community,
                    &room.system_channel,
                    &room.created,
                    &room.known_complete,
                    &room.is_dm,
                    &room.user_a,
                    &room.user_b,
                    &room.info.name,
                    &room.info.description,
                    &room.info.category,
                    &room.info.display_order,
                ],
            )
            .await
            .expect("failed to create room")
            .pop()
            .expect("creating room returned nothing");
        result.into()
    }
    pub async fn get_room(&self, room_id: &Uuid) -> Option<Room> {
        let result = self
            .query(Room::read_statement(), &[room_id])
            .await
            .expect("failed to fetch room")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn update_room(&self, room: Room) -> Room {
        let result = self
            .query(
                Room::update_statement(),
                &[
                    &room.external_id,
                    &room.domain,
                    &room.system_channel,
                    &room.info.name,
                    &room.info.description,
                    &room.known_complete,
                    &room.info.category,
                    &room.info.display_order,
                    &room.id,
                ],
            )
            .await
            .expect("failed to update room")
            .pop()
            .expect("updating room returned nothing");
        result.into()
    }
    pub async fn delete_room(&self, room_id: &Uuid) {
        let _result = self
            .query(Room::delete_statement(), &[room_id])
            .await
            .expect("failed to delete room");
    }
    pub async fn get_all_comm_rooms(&self, com_id: &Uuid) -> Vec<Room> {
        let result = self
            .query(Room::get_all_comm_rooms(), &[com_id])
            .await
            .expect("failed to fetch community rooms");
        result.into_iter().map(|x| x.into()).collect()
    }
}

// ------------------------- message -----------------------------
#[allow(dead_code)]
impl Sesh<'_> {
    pub async fn create_message(&self, message: DbMessage) -> DbMessage {
        let result = self
            .query(
                DbMessage::create_statement(),
                &[
                    &message.id,
                    &message.external_id,
                    &message.domain,
                    &message.user,
                    &message.info.room,
                    &message.published,
                    &message.edited,
                    &message.fetched_at,
                    &message.info.is_reply,
                    &message.info.in_reply_to,
                    &message.info.content,
                    &message.info.format.as_str(),
                    &message.info.language.map(|x| x.to_string()),
                ],
            )
            .await
            .expect("failed to create message")
            .pop()
            .expect("creating message returned nothing");
        result.into()
    }
    pub async fn get_message(&self, m_id: &Uuid) -> Option<DbMessage> {
        let result = self
            .query(DbMessage::read_statement(), &[m_id])
            .await
            .expect("failed to fetch message")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn get_api_message(&self, m_id: &Uuid) -> Option<ApiMessage> {
        let result = self
            .query(DbMessage::read_joined_statement(), &[m_id])
            .await
            .expect("failed to fetch message")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn update_message(&self, message: DbMessage) -> DbMessage {
        let result = self
            .query(
                DbMessage::update_statement(),
                &[
                    &message.edited,
                    &message.info.content,
                    &message.info.format.as_str(),
                    &message.info.language.map(|x| x.to_string()),
                    &message.fetched_at,
                ],
            )
            .await
            .expect("failed to update message")
            .pop()
            .expect("updating message returned nothing");
        result.into()
    }
    pub async fn delete_message(&self, m_id: &Uuid) {
        let _result = self
            .query(DbMessage::delete_statement(), &[m_id])
            .await
            .expect("failed to delete message");
    }
    pub async fn get_room_messages(&self, room_id: &Uuid, limit: i64) -> Vec<ApiMessage> {
        let result = self
            .query(DbMessage::get_room_messages(), &[room_id, &limit])
            .await
            .expect("failed to fetch room messages");
        result.into_iter().map(|x| x.into()).collect()
    }
    pub async fn get_room_messages_before(
        &self,
        room_id: Uuid,
        limit: i64,
        time: i64,
        post: Uuid,
    ) -> Vec<ApiMessage> {
        let result = self
            .query(
                DbMessage::get_messages_prior(),
                &[&room_id, &time, &post, &limit],
            )
            .await
            .expect("failed to fetch room messages");
        result.into_iter().map(|x| x.into()).collect()
    }
}
