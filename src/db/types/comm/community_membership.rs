use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommMembership {
    pub com_id: Uuid,
    pub uid: Uuid,
    pub joined: i64,
    pub owner: bool,
}

impl From<tokio_postgres::Row> for CommMembership {
    fn from(row: tokio_postgres::Row) -> Self {
        CommMembership {
            com_id: row.get("com_id"),
            uid: row.get("uid"),
            joined: row.get("joined"),
            owner: row.get("owner"),
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
            joined,
            owner
        )
        VALUES
        (
            $1, $2, $3, $4
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
    // /// note no trigger yet
    // pub const fn update_owner_statement() -> &'static str {
    //     r#"
    //     UPDATE community_membership SET
    //     owner = $1,
    //     WHERE com_id = $2 AND uid = $3
    //     RETURNING *;
    //     "#
    // }
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
