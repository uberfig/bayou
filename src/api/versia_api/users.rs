use crate::{
    cryptography::digest::sha256_hash,
    db::conn::{Conn, EntityOrigin},
    protocol::{
        headers::ActixHeaders,
        versia_protocol::{signatures::HttpMethod, verify::verify_request},
    },
};
use actix_web::{
    dev::ResourcePath,
    error::{ErrorNotFound, ErrorUnauthorized},
    get,
    web::Data,
    HttpRequest, HttpResponse, Result,
};

#[get("/users/{uuid}/versia")]
pub async fn versia_user(
    request: HttpRequest,
    body: actix_web::web::Bytes,
    actix_path: actix_web::web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<Box<dyn Conn + Sync>>,
) -> Result<HttpResponse> {
    let path = actix_path.path().to_string();
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
        &conn,
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