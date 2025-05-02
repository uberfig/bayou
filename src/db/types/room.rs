use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub id: Uuid,
    pub external_id: Uuid,
    pub domain: String,
    pub community: Option<Uuid>,
    pub system_channel: bool,
    pub created: i64,
    pub known_complete: bool,
    pub is_dm: bool,
    /// if this is a dm or group chat this is the user that started it
    /// we do this so that it can be automatically deleted if that user
    /// is deleted and we are able to query for dms
    pub user_a: Option<Uuid>,
    /// only used for direct messages, user B will not be the one to
    /// init the chat. exists so it will be auto deleted if they
    /// are deleted and so they can query for dms
    pub user_b: Option<Uuid>,
    pub info: RoomInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoomInfo {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<Uuid>,
    pub display_order: i64,
}

impl From<tokio_postgres::Row> for Room {
    fn from(row: tokio_postgres::Row) -> Self {
        Room {
            id: row.get("room_id"),
            external_id: row.get("external_id"),
            domain: row.get("domain"),
            community: row.get("community"),
            system_channel: row.get("system_channel"),
            info: RoomInfo {
                name: row.get("name"),
                description: row.get("description"),
                category: row.get("category"),
                display_order: row.get("display_order"),
            },
            created: row.get("created"),
            known_complete: row.get("known_complete"),
            is_dm: row.get("is_dm"),
            user_a: row.get("user_a"),
            user_b: row.get("user_b"),
        }
    }
}

impl Room {
    pub const fn create_statement() -> &'static str {
        r#"
        INSERT INTO rooms 
        (
            room_id,
            external_id,
            domain,
            community,
            system_channel,
            created,
            known_complete,
            is_dm,
            user_a,
            user_b,
            name,
            description,
            category,
            display_order
        )
        VALUES
        (
            $1, $2, $3, $4, $5, $6, $7, $8,
            $9, $10, $11, $12, $13, $14
        )
        RETURNING *;
        "#
    }
    pub const fn read_statement() -> &'static str {
        r#"
        SELECT * FROM rooms WHERE room_id = $1;
        "#
    }
    pub const fn update_statement() -> &'static str {
        r#"
        UPDATE rooms SET
        external_id = $1,
        domain = $2,
        system_channel = $3,
        name = $4,
        description = $5,
        known_complete = $7
        category = $7,
        display_order = $8

        WHERE room_id = $9
        RETURNING *;
        "#
    }
    pub const fn delete_statement() -> &'static str {
        r#"
        DELETE FROM rooms WHERE room_id = $1;
        "#
    }
    pub const fn get_all_comm_rooms() -> &'static str {
        r#"
        SELECT * FROM rooms WHERE community = $1;
        "#
    }
}
