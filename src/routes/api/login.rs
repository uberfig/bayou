use actix_web::{
    post,
    web::Data,
    HttpResponse, Result,
};

use crate::db::pg_conn::PgConn;

#[post("/login")]
async fn login(
    state: Data<crate::config::Config>,
    conn: Data<PgConn>,
) -> Result<HttpResponse> {
    todo!()
}
