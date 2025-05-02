use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Result,
};
use serde::{Deserialize, Serialize};

use crate::{
    cryptography::passwords::verify_password, db::pg_conn::PgConn,
    routes::api::types::login_request::LoginRequest,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoginErr {
    InvalidUsernameOrPassword,
    InvalidDevice,
}

fn invalid(err: LoginErr) -> Result<HttpResponse> {
    Ok(HttpResponse::BadRequest()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&err).expect("failed to serialize LoginErr")))
}

#[post("/login")]
pub async fn login(
    state: Data<crate::config::Config>,
    conn: Data<PgConn>,
    login_request: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let Some(user) = conn
        .get_user(&login_request.username, &state.instance_domain)
        .await
    else {
        return invalid(LoginErr::InvalidUsernameOrPassword);
    };
    let Some(local_info) = user.local_info else {
        return invalid(LoginErr::InvalidUsernameOrPassword);
    };
    if !verify_password(login_request.password.as_bytes(), &local_info.password) {
        return invalid(LoginErr::InvalidUsernameOrPassword);
    }
    let Some(device) = conn.get_registered_device(&login_request.device_id).await else {
        return invalid(LoginErr::InvalidDevice);
    };
    let token = conn.create_auth_token(&device.device_id, &user.id).await;
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&token).expect("failed to serialize login token")))
}
