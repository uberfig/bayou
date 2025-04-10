use super::{
    curr_time::{get_current_time, get_expiry},
    types::{
        auth_token::AuthToken,
        instance::Instance,
        registered_device::{DeviceInfo, RegisteredDevice},
        signup_token::SignupToken,
        user::{DbUser, UserInfo},
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
impl Sesh<'_> {
    pub async fn get_user(&self, username: &str, domain: &str) -> Option<DbUser> {
        let result = self
            .query(DbUser::read_statement(), &[&username, &domain])
            .await
            .expect("failed to fetch user")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn create_user(&self, new_user: UserInfo) -> DbUser {
        let id = Uuid::now_v7();
        let result = self
            .query(
                DbUser::create_statement(),
                &[
                    &id,
                    &new_user.domain,
                    &new_user.username,
                    &new_user.display_name,
                    &new_user.summary,
                    &new_user.custom_emoji,
                    &new_user.banned,
                    &new_user.reason,
                    &new_user.fetched_at,
                    &new_user.local_info.is_some(),
                    &new_user.local_info.as_ref().map(|x| x.password.clone()),
                    &new_user.local_info.as_ref().map(|x| x.email.clone()),
                    &new_user.local_info.as_ref().map(|x| x.verified),
                    &new_user.local_info.as_ref().map(|x| x.is_admin),
                    &new_user.local_info.as_ref().map(|x| x.instance_mod),
                    &new_user.created,
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
                    &user.info.custom_emoji,
                    &user.info.banned,
                    &user.info.reason,
                    &user.info.fetched_at,
                    &user.info.local_info.is_some(),
                    &user.info.local_info.as_ref().map(|x| x.password.clone()),
                    &user.info.local_info.as_ref().map(|x| x.email.clone()),
                    &user.info.local_info.as_ref().map(|x| x.verified),
                    &user.info.local_info.as_ref().map(|x| x.is_admin),
                    &user.info.local_info.as_ref().map(|x| x.instance_mod),
                    &user.info.created,
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
            .expect("failed to update tag")
            .pop()
            .expect("updating tag returned nothing");
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
impl Sesh<'_> {
    pub async fn create_auth_token(&self, device: &Uuid, user: &Uuid) -> AuthToken {
        let id = Uuid::new_v4();
        let expiry = get_expiry(60);
        let result = self
            .query(
                AuthToken::create_statement(),
                &[&id, &device, &user, &expiry],
            )
            .await
            .expect("failed to create auth token")
            .pop()
            .expect("creating auth token returned nothing");
        result.into()
    }
    pub async fn get_auth_token(&self, token_id: &Uuid) -> Option<AuthToken> {
        let result = self
            .query(AuthToken::read_statement(), &[token_id])
            .await
            .expect("failed to fetch auth token")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn delete_auth_token(&self, token_id: &Uuid) {
        let _result = self
            .query(AuthToken::delete_statement(), &[token_id])
            .await
            .expect("failed to delete registered device");
    }
}
