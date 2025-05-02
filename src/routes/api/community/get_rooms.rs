use crate::{db::pg_conn::PgConn, routes::api::types::info_with_token::BearrerWithInfo};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Result,
};
use uuid::Uuid;

#[get("/rooms")]
async fn get_rooms(
    conn: Data<PgConn>,
    community: web::Json<BearrerWithInfo<Uuid>>,
) -> Result<HttpResponse> {
    if conn.validate_auth_token(&community.token).await.is_err() {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    }
    let Ok(rooms) = conn
        .get_comm_rooms(community.info, community.token.uid)
        .await
    else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&rooms).expect("failed to serialize dbcommunity")))
}
