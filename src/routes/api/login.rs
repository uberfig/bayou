use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Result,
};

use crate::{db::pg_conn::PgConn, routes::api::types::login_request::LoginRequest};

pub enum LoginErr {
    InvalidUsernameOrPassword
}

#[post("/login")]
async fn login(
    state: Data<crate::config::Config>,
    conn: Data<PgConn>,
    login_request: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let Some(user) = conn.get_user(&login_request.username, &state.instance_domain).await else {
        todo!()
    };
    todo!()
}
