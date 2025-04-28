use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommMembership {
    pub com_id: Uuid,
    pub uid: Uuid,
    pub joined: i64,
}

impl From<tokio_postgres::Row> for CommMembership {
    fn from(row: tokio_postgres::Row) -> Self {
        CommMembership {
            com_id: row.get("com_id"),
            uid: row.get("uid"),
            joined: row.get("joined"),
        }
    }
}

impl CommMembership {
    /// params:
    /// - $1: com_id
    /// - $2: uid
    /// - $3: joined
    /// - $4: owner
    pub const fn create_statement() -> &'static str {
        r#"
        INSERT INTO community_membership 
        (
            com_id,
            uid,
            joined
        )
        VALUES
        (
            $1, $2, $3
        )
        RETURNING *;
        "#
    }
    /// params:
    /// - $1: com_id
    /// - $2: uid
    pub const fn read_statement() -> &'static str {
        r#"
        SELECT * FROM community_membership WHERE com_id = $1 AND uid = $2;
        "#
    }
    /// params:
    /// - $1: com_id
    /// - $2: uid
    pub const fn delete_statement() -> &'static str {
        r#"
        DELETE FROM community_membership WHERE com_id = $1 AND uid = $2;
        "#
    }

    pub const fn get_all_user_comms() -> &'static str {
        r#"
        SELECT * FROM community_membership WHERE uid = $1;
        "#
    }
    pub const fn get_all_comm_members() -> &'static str {
        r#"
        SELECT * FROM community_membership WHERE com_id = $1;
        "#
    }
}
