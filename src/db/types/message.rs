use uuid::Uuid;

pub struct Message {
    pub id: Uuid,
    pub external_id: Uuid,
    pub domain: String,
    pub user: Uuid,
    pub room: Uuid,
    pub published: i64,
    pub is_reply: bool,
    pub in_reply_to: Uuid,
    pub content: String,
}

impl From<tokio_postgres::Row> for Message {
    fn from(row: tokio_postgres::Row) -> Self {
        Message {
            id: row.get("id"),
            external_id: row.get("external_id"),
            domain: row.get("domain"),
            user: row.get("user"),
            room: row.get("room"),
            published: row.get("published"),
            is_reply: row.get("is_reply"),
            in_reply_to: row.get("in_reply_to"),
            content: row.get("content"),
        }
    }
}
