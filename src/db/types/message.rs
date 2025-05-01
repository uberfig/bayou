use std::str::FromStr;

use codes_iso_639::part_1::LanguageCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbMessage {
    pub id: Uuid,
    pub external_id: Uuid,
    pub domain: String,
    pub user: Uuid,
    pub published: i64,
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

impl From<tokio_postgres::Row> for DbMessage {
    fn from(row: tokio_postgres::Row) -> Self {
        let language: Option<&str> = row.get("language");
        let language = language.map(|x| LanguageCode::from_str(x).ok()).flatten();
        DbMessage {
            id: row.get("id"),
            external_id: row.get("external_id"),
            domain: row.get("domain"),
            user: row.get("user"),
            published: row.get("published"),

            info: Messageinfo {
                room: row.get("room"),
                is_reply: row.get("is_reply"),
                in_reply_to: row.get("in_reply_to"),
                content: row.get("content"),
                proxy_id: row.get("proxy_id"),
                format: TextFormat::from_str(row.get("proxy_id"))
                    .expect("unkown text format in db"),
                language,
            },
        }
    }
}
