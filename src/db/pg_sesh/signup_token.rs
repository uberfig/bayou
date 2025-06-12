use uuid::Uuid;

use crate::db::{pg_sesh::Sesh, types::{tokens::signup_token::SignupToken, user::DbUser}};

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