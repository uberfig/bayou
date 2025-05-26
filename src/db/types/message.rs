use std::str::FromStr;

use codes_iso_639::part_1::LanguageCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::api::types::api_message::ApiMessage;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TextFormat {
    Markdown,
    Plain,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum FormatErr {
    Unkown,
}

impl FromStr for TextFormat {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "markdown" => Ok(Self::Markdown),
            "plain" => Ok(Self::Plain),
            _ => Err(FormatErr::Unkown),
        }
    }
}
impl TextFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextFormat::Markdown => "markdown",
            TextFormat::Plain => "plain",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbMessage {
    pub id: Uuid,
    pub external_id: Uuid,
    pub domain: String,
    pub user: Uuid,
    pub published: i64,
    pub edited: Option<i64>,
    pub fetched_at: Option<i64>,
    pub info: Messageinfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Messageinfo {
    /// we seperate is reply and in reply to
    /// so that if a message is in reply to something
    /// but the origional is deleted or not federated
    /// clients can just say in reply to "removed or
    /// not federated"
    pub is_reply: bool,
    pub in_reply_to: Option<Uuid>,
    /// users can optionally have proxies that behave
    /// like pluralkit. Users may only use proxies that
    /// they created and clients can decide how to display
    /// proxy messages
    pub proxy_id: Option<Uuid>,
    pub content: String,
    pub format: TextFormat,
    pub language: Option<LanguageCode>,
    pub room: Uuid,
}

impl From<tokio_postgres::Row> for ApiMessage {
    /// note, requires being joined on the users table and on the proxy table in the future
    fn from(row: tokio_postgres::Row) -> Self {
        let language: Option<&str> = row.get("language");
        let language = language.map(|x| LanguageCode::from_str(x).ok()).flatten();
        ApiMessage {
            id: row.get("m_id"),
            room: row.get("room_id"),
            published: row.get("published"),
            edited: row.get("edited"),
            is_reply: row.get("is_reply"),
            in_reply_to: row.get("in_reply_to"),
            content: row.get("content"),
            proxy_id: row.get("proxy_id"),
            format: TextFormat::from_str(row.get("format")).expect("unkown text format in db"),
            language,
            user: row.into(),
        }
    }
}

impl From<tokio_postgres::Row> for DbMessage {
    fn from(row: tokio_postgres::Row) -> Self {
        let language: Option<&str> = row.get("language");
        let language = language.map(|x| LanguageCode::from_str(x).ok()).flatten();
        DbMessage {
            id: row.get("m_id"),
            external_id: row.get("external_id"),
            domain: row.get("domain"),
            user: row.get("uid"),
            published: row.get("published"),
            edited: row.get("edited"),
            fetched_at: row.get("fetched_at"),

            info: Messageinfo {
                room: row.get("room_id"),
                is_reply: row.get("is_reply"),
                in_reply_to: row.get("in_reply_to"),
                content: row.get("content"),
                proxy_id: row.get("proxy_id"),
                format: TextFormat::from_str(row.get("format")).expect("unkown text format in db"),
                language,
            },
        }
    }
}

impl DbMessage {
    pub const fn create_statement() -> &'static str {
        r#"
        INSERT INTO messages 
        (
            m_id,
            external_id,
            domain,
            uid,
            room_id,
            published,
            edited,
            fetched_at,
            is_reply,
            in_reply_to,
            content,
            format,
            language
        )
        VALUES
        (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
        )
        RETURNING *;
        "#
    }
    pub const fn read_statement() -> &'static str {
        r#"
        SELECT * FROM messages WHERE m_id = $1;
        "#
    }
    pub const fn read_joined_statement() -> &'static str {
        r#"
        SELECT * FROM messages INNER JOIN users USING (uid, domain) WHERE m_id = $1;
        "#
    }
    pub const fn update_statement() -> &'static str {
        r#"
        UPDATE messages
        SET
            edited = $1,
            content = $2,
            format = $3,
            language = $4,
            fetched_at = $5
        WHERE
            m_id = $6
        RETURNING *;
        "#
    }
    pub const fn delete_statement() -> &'static str {
        r#"
        DELETE FROM messages WHERE m_id = $1;
        "#
    }
    pub const fn get_room_messages() -> &'static str {
        r#"
        SELECT * FROM messages INNER JOIN users USING (uid, domain) WHERE room_id = $1 ORDER BY published DESC LIMIT $2;
        "#
    }
    pub const fn get_messages_prior() -> &'static str {
        r#"
        SELECT * FROM messages NATURAL INNER JOIN users WHERE room_id = $1 AND published <= $2 AND m_id <> $3 ORDER BY published DESC LIMIT $4;
        "#
    }
}
