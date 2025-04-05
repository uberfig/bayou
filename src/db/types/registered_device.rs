use uuid::Uuid;

pub struct RegisteredDevice {
    pub device_id: Uuid,
    pub info: DeviceInfo,
}

pub struct DeviceInfo {
    pub device_name: Option<String>,
    pub software: Option<String>,
    pub webpage: Option<String>,
    pub redirect_url: Option<String>,
    pub registered_at: i64,
}

impl From<tokio_postgres::Row> for RegisteredDevice {
    fn from(row: tokio_postgres::Row) -> Self {
        RegisteredDevice {
            device_id: row.get("device_id"),
            info: DeviceInfo {
                device_name: row.get("device_name"),
                software: row.get("software"),
                webpage: row.get("webpage"),
                redirect_url: row.get("redirect_url"),
                registered_at: row.get("registered_at"),
            },
        }
    }
}

impl RegisteredDevice {
    pub const fn create_statement() -> &'static str {
        r#"
        INSERT INTO registered_devices
        (device_id, device_name, software, webpage, redirect_url, registered_at)
        VALUES
        ($1, $2, $3, $4, $5, $6)
        RETURNING *;
        "#
    }
    pub const fn read_statement() -> &'static str {
        r#"
        SELECT * FROM registered_devices WHERE device_id = $1;
        "#
    }
    pub const fn delete_statement() -> &'static str {
        r#"
        DELETE FROM registered_devices WHERE device_id = $1;
        "#
    }
}
