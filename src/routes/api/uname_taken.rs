use crate::db::pg_conn::PgConn;
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Result,
};

/// `get /api/bayou_v1/username_availible`
///
/// request with a username in the body and it will check if it has been taken
/// responses:
/// - ok (200) username is available
/// - conflict (409) username is taken
#[get("/username_availible")]
pub async fn signup(
    state: Data<crate::config::Config>,
    conn: Data<PgConn>,
    new_user: web::Json<String>,
) -> Result<HttpResponse> {
    match conn
        .username_taken(&new_user.into_inner(), &state.instance_domain)
        .await
    {
        false => Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .body("")),
        true => Ok(HttpResponse::Conflict()
            .content_type("application/json; charset=utf-8")
            .body("")),
    }
}
