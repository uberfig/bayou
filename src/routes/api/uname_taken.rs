//! `get /api/bayou_v1/username_availible/{uname}`
//! 
//! will check if it has been taken
//! responses:
//! - ok (200) username is available
//! - conflict (409) username is taken

use crate::db::pg_conn::PgConn;
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Result,
};

#[get("/username_availible/{uname}")]
pub async fn username_availible(
    state: Data<crate::config::Config>,
    conn: Data<PgConn>,
    path: web::Path<String>
) -> Result<HttpResponse> {
    match conn
        .username_taken(&path.into_inner(), &state.instance_domain)
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
