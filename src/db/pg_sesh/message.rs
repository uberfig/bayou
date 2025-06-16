use uuid::Uuid;

use crate::{
    db::{pg_sesh::Sesh, types::message::DbMessage},
    routes::api::types::api_message::ApiMessage,
};

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
    /// warning, this can be multiple operations for getting the preview
    pub async fn get_api_message(&self, m_id: &Uuid) -> Option<ApiMessage> {
        let result = self
            .query(DbMessage::read_joined_statement(), &[m_id])
            .await
            .expect("failed to fetch message")
            .pop();
        match result {
            Some(row) => {
                let json: tokio_postgres::types::Json<ApiMessage> = row.get("json_build_object");
                Some(json.0)
            }
            None => None,
        }
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

        result
            .into_iter()
            .rev()
            .map(|x| {
                let json: tokio_postgres::types::Json<ApiMessage> = x.get("json_build_object");
                json.0
            })
            .collect()
    }
    pub async fn get_room_messages_before(
        &self,
        room_id: Uuid,
        limit: i64,
        post: Uuid,
        inclusive: bool,
    ) -> Vec<ApiMessage> {
        let result = self
            .query(
                DbMessage::get_messages_prior(inclusive),
                &[&room_id, &post, &limit],
            )
            .await
            .expect("failed to fetch room messages");
        // we rev to make oldest to newest as the api expects
        result
            .into_iter()
            .rev()
            .map(|x| {
                let json: tokio_postgres::types::Json<ApiMessage> = x.get("json_build_object");
                json.0
            })
            .collect()
    }
    pub async fn get_room_messages_after(
        &self,
        room_id: Uuid,
        limit: i64,
        post: Uuid,
        inclusive: bool,
    ) -> Vec<ApiMessage> {
        let result = self
            .query(
                DbMessage::get_messages_after(inclusive),
                &[&room_id, &post, &limit],
            )
            .await
            .expect("failed to fetch room messages");
        // this query is in the right order
        result
            .into_iter()
            .map(|x| {
                let json: tokio_postgres::types::Json<ApiMessage> = x.get("json_build_object");
                json.0
            })
            .collect()
    }
    // pub async fn get_reply_preview(&self, m_id: Uuid) -> Option<ReplyPreview> {
    //     let result = self
    //         .query(DbMessage::read_statement(), &[&m_id])
    //         .await
    //         .expect("failed to fetch message")
    //         .pop();
    //     match result {
    //         Some(row) => Some(row.into()),
    //         None => None,
    //     }
    // }
}
