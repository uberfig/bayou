use super::types::{
    instance::Instance,
    user::{DbUser, UserInfo},
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

//users
impl Sesh<'_> {
    pub async fn get_user(&self, username: &str, domain: &str) -> Option<DbUser> {
        let stmt = r#"
            SELECT * FROM users WHERE username = $1 AND domain = $2;
        "#;
        let result = self
            .query(stmt, &[&username, &domain])
            .await
            .expect("failed to fetch user")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn create_user(
        &self,
        new_user: UserInfo,
    ) -> DbUser {
        let id = Uuid::now_v7();
        let stmt = r#"
        INSERT INTO users 
        (
            uid,
            domain,
            username,
            display_name,
            summary,
            custom_emoji,
            banned,
            reason,
            fetched_at,
            is_authoratative,
            password,
            email,
            verified,
            is_admin,
            instance_mod,
            created
        )
        VALUES
        (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 
            $11, $12, $13, $14, $15, $16
        )
        RETURNING *;
        "#;
        let result = self
            .query(
                stmt,
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
        let stmt = r#"
        UPDATE users SET
        display_name = $1,
        summary = $2,
        custom_emoji = $3,
        banned = $4,
        reason = $5,
        fetched_at = $6,
        is_authoratative = $7,
        password = $8,
        email = $9,
        verified = $10,
        is_admin = $11,
        instance_mod = $12,
        created = $13
        WHERE uid = $14
        RETURNING *;
        "#;
        let result = self
            .query(
                stmt,
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
        todo!()
    }
    pub async fn set_user_banned(&self, user: &DbUser, banned: bool, reason: Option<String>) {
        todo!()
    }
    pub async fn username_taken(&self, domain: &str) -> bool {
        todo!()
    }
}

impl Sesh<'_> {
    pub async fn create_instance(
        &self,
        domain: &str,
        is_authoratative: bool,
        banned: bool,
        reason: Option<String>,
        allowlist: bool,
    ) -> Instance {
        let stmt = r#"
        INSERT INTO instances
        (domain, is_authoratative, blocked, reason, allowlisted)
        VALUES
        ($1, $2, $3, $4, $5)
        RETURNING *;
        "#;
        let result = self
            .query(
                stmt,
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
        let stmt = r#"
        UPDATE instances SET
        is_authoratative = $1,
        blocked = $2,
        reason = $3,
        allowlisted = $4
        WHERE domain = $5
        RETURNING *;
        "#;
        let result = self
            .query(
                stmt,
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
        todo!()
    }
}
