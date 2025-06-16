use uuid::Uuid;

use crate::db::{pg_sesh::Sesh, types::comm::community::DbCommunity};

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
