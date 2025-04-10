use uuid::Uuid;

pub struct Community {
    pub id: Uuid,
    pub external_id: Uuid,
    pub domain: String,
    pub name: String,
    pub description: Option<String>,
    pub created: i64,
}

impl From<tokio_postgres::Row> for Community {
    fn from(row: tokio_postgres::Row) -> Self {
        Community {
            id: row.get("id"),
            external_id: row.get("external_id"),
            domain: row.get("domain"),
            name: row.get("name"),
            description: row.get("description"),
            created: row.get("created"),
        }
    }
}

impl Community {
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
            created
        )
        VALUES
        (
            $1, $2, $3, $4, $5, $6, $7
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