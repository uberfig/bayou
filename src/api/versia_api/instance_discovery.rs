use crate::db::dbconn::DbConn;
use actix_web::{get, web::Data, HttpResponse, Result};

#[get("/.well-known/versia")]
pub async fn versia_metadata(
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    let metadata = conn
        .get_versia_instance_metadata(&state.instance_domain)
        .await;
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=UTF-8")
        .body(serde_json::to_string(&metadata).unwrap()))
}
