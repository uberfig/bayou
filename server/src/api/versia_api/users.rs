use crate::db::conn::Conn;
use crate::{
    api::headers::ActixHeaders,
    db::{conn::EntityOrigin, dbconn::DbConn},
};
use actix_web::{
    error::{ErrorNotFound, ErrorUnauthorized},
    get,
    web::Data,
    HttpRequest, HttpResponse, Result,
};
use bayou_protocol::{
    cryptography::digest::sha256_hash,
    protocol::{http_method::HttpMethod, versia_protocol::verify::verify_request},
};

#[get("/users/{uuid}/versia")]
pub async fn versia_user(
    request: HttpRequest,
    body: actix_web::web::Bytes,
    actix_path: actix_web::web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    let path = request.path();
    let uuid = actix_path.into_inner();

    let Ok(body) = String::from_utf8(body.to_vec()) else {
        return Err(ErrorUnauthorized("bad request body"));
    };
    let hash = sha256_hash(body.as_bytes());

    let authorized = verify_request(
        &ActixHeaders {
            headermap: request.headers().clone(),
        },
        HttpMethod::Get,
        &path,
        &hash,
        &**conn,
    )
    .await;

    if let Err(err) = authorized {
        return Err(ErrorUnauthorized(err));
    }

    let user = conn
        .get_versia_user(&uuid, &EntityOrigin::Local(&state.instance_domain))
        .await;

    match user {
        Some(x) => Ok(HttpResponse::Ok()
            .content_type("application/json; charset=UTF-8")
            .body(serde_json::to_string(&x).unwrap())),
        None => Err(ErrorNotFound(r#"{"error":"Not Found"}"#)),
    }
}