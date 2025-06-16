use uuid::Uuid;

use crate::db::{pg_sesh::Sesh, types::user::DbUser};

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
