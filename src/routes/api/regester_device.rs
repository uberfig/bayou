use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Result,
};

use crate::db::{pg_conn::PgConn, types::registered_device::DeviceInfo};

#[post("/register")]
async fn register_device(
    conn: Data<PgConn>,
    device_info: web::Json<DeviceInfo>,
) -> Result<HttpResponse> {
    let device = conn
        .create_registered_device(&device_info.into_inner())
        .await;
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&device).expect("failed to serialize registered device")))
}
