use uuid::Uuid;

use crate::db::{curr_time::get_expiry, pg_sesh::Sesh, types::tokens::auth_token::DBAuthToken};

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
