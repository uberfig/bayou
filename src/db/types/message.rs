use std::str::FromStr;

use codes_iso_639::part_1::LanguageCode;
use const_format::formatcp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::api::types::{
    api_message::ReplyPreview, api_user::ApiUser, proxy_user::ApiProxyUser,
};

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
            "Markdown" => Ok(Self::Markdown),
            "Plain" => Ok(Self::Plain),
            _ => Err(FormatErr::Unkown),
        }
    }
}
impl TextFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextFormat::Markdown => "Markdown",
            TextFormat::Plain => "Plain",
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

impl From<tokio_postgres::Row> for ReplyPreview {
    fn from(row: tokio_postgres::Row) -> Self {
        let language: Option<&str> = row.get("language");
        let language = language.map(|x| LanguageCode::from_str(x).ok()).flatten();
        Self {
            id: row.get("m_id"),
            proxy: ApiProxyUser::maybe_from_row(&row),
            content: row.get("content"),
            format: TextFormat::from_str(row.get("format")).expect("unkown text format in db"),
            language,
            user: ApiUser::from_row(&row),
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

// const SELECT_JOINED: &str =
// r#"SELECT * FROM messages
//     INNER JOIN users USING (uid, domain) LEFT JOIN proxies USING (uid, proxy_id)"#;

const SELECT_JOINED: &str = r#"SELECT
json_build_object(
	'id', main.m_id,
	'room', main.room_id,
	'user', json_build_object(
		'id', u.uid,
		'domain', u.domain,
		'username', u.username,
		'display_name', u.display_name,
		'summary', u.summary,
		'created', u.created
	),
	'published', main.published,
	'edited', main.edited,
	'is_reply', main.is_reply,
	'proxy', CASE
		when p.proxy_id is null then null
		else
			json_build_object(
			'id', p.proxy_id,
			'name', p.proxy_name,
			'bio', p.proxy_bio,
			'created', p.proxy_created,
			'parent_id', p.uid
			)
		end
	,
	'preview', CASE
		when prev.m_id is null then null
		else
			json_build_object(
				'id', prev.m_id,
				'user', json_build_object(
					'id', prev.uid,
					'domain', prev.domain,
					'username', prev.username,
					'display_name', prev.display_name,
					'summary', prev.summary,
					'created', prev.created
				),
				'proxy', CASE
					when prev.proxy_id is null then null
					else
						json_build_object(
						'id', prev.proxy_id,
						'name', prev.proxy_name,
						'bio', prev.proxy_bio,
						'created', prev.proxy_created,
						'parent_id', prev.uid
						)
					end
				,
				'content', prev.content,
				'format', prev.format,
				'language', prev.language
			)
		end,
	'content', main.content,
	'format', main.format,
	'language', main.language
)
FROM 
messages main
    INNER JOIN 
        users u USING (uid, domain) 
    LEFT JOIN 
        proxies p USING (uid, proxy_id)
LEFT JOIN
(
    messages  
    INNER JOIN 
        users USING (uid, domain) 
    LEFT JOIN 
        proxies USING (uid, proxy_id)
) prev ON main.in_reply_to = prev.m_id"#;

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
        formatcp!(r#"{} WHERE main.m_id = $1;"#, SELECT_JOINED)
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
        formatcp!(
            r#"{} WHERE main.room_id = $1 ORDER BY main.published DESC LIMIT $2;"#,
            SELECT_JOINED
        )
    }
    /// gets messages older than a given message, messages in order of newest to oldest
    /// 1. room_id
    /// 2. m_id
    /// 3. LIMIT
    pub const fn get_messages_prior(inclusive: bool) -> &'static str {
        match inclusive {
            true => {
                formatcp!(
                    r#"{} WHERE main.room_id = $1 AND main.m_id <= $2 ORDER BY main.published DESC LIMIT $3;"#,
                    SELECT_JOINED
                )
            }
            false => {
                formatcp!(
                    r#"{} WHERE main.room_id = $1 AND main.m_id < $2 ORDER BY main.published DESC LIMIT $3;"#,
                    SELECT_JOINED
                )
            }
        }
    }
    /// gets messages newer than a given message, messages in order of oldest to newest
    /// 1. room_id
    /// 2. m_id
    /// 3. LIMIT
    pub const fn get_messages_after(inclusive: bool) -> &'static str {
        match inclusive {
            true => {
                formatcp!(
                    r#"{} WHERE main.room_id = $1 AND main.m_id >= $2 ORDER BY main.published ASC LIMIT $3;"#,
                    SELECT_JOINED
                )
            }
            false => {
                formatcp!(
                    r#"{} WHERE main.room_id = $1 AND main.m_id > $2 ORDER BY main.published ASC LIMIT $3;"#,
                    SELECT_JOINED
                )
            }
        }
    }
}
