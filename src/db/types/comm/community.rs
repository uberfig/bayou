use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbCommunity {
    pub id: Uuid,
    /// will be equal to the id when a local community, used to
    /// access communities at the protocol endpoint when federation
    /// is implimented
    pub external_id: Uuid,
    pub domain: String,
    pub info: Communityinfo,
    pub created: i64,
    pub owner: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Communityinfo {
    pub name: String,
    pub description: Option<String>,
    /// custom emoji present in the name and description
    pub custom_emoji: Option<String>,
}

impl From<tokio_postgres::Row> for DbCommunity {
    fn from(row: tokio_postgres::Row) -> Self {
        DbCommunity {
            id: row.get("id"),
            external_id: row.get("external_id"),
            domain: row.get("domain"),
            info: Communityinfo {
                name: row.get("name"),
                description: row.get("description"),
                custom_emoji: row.get("custom_emoji"),
            },
            created: row.get("created"),
            owner: row.get("owner"),
        }
    }
}

impl DbCommunity {
    pub const fn create_statement() -> &'static str {
        r#"
        INSERT INTO communities 
        (
            com_id,
            external_id,
            domain,
            name,
            description,
            custom_emoji,
            created,
            owner
        )
        VALUES
        (
            $1, $2, $3, $4, $5, $6, $7, $8
        )
        RETURNING *;
        "#
    }
    pub const fn read_statement() -> &'static str {
        r#"
        SELECT * FROM communities WHERE com_id = $1;
        "#
    }
    pub const fn update_statement() -> &'static str {
        r#"
        UPDATE communities SET
        name = $1,
        description = $2,
        custom_emoji = $3
        WHERE com_id = $4
        RETURNING *;
        "#
    }
    pub const fn delete_statement() -> &'static str {
        r#"
        DELETE FROM communities WHERE com_id = $1;
        "#
    }
}
