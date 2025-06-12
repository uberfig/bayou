use uuid::Uuid;

use crate::db::{pg_sesh::Sesh, types::{comm::community_membership::CommMembership, user::DbUser}};

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