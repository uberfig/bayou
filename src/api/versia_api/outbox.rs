use crate::{
    api::{headers::ActixHeaders, page_query::Page},
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
    types::versia_types::structures::collection::Collection,
};

#[get("/users/{uuid}/outbox")]
pub async fn versia_outbox(
    request: HttpRequest,
    body: actix_web::web::Bytes,
    actix_path: actix_web::web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
    page: actix_web::web::Query<Page>,
) -> Result<HttpResponse> {
    let page = page.page.unwrap_or(1);
    if page.eq(&0) {
        return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
    }

    let uuid = actix_path.into_inner();
    let path = request.path();

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

    let Some(count) = conn
        .get_user_post_count(&uuid, &EntityOrigin::Local(&state.instance_domain))
        .await
    else {
        return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
    };

    let posts = conn
        .get_user_posts_versia(
            &uuid,
            &EntityOrigin::Local(&state.instance_domain),
            state.outbox_pagnation_size,
            page,
        )
        .await;

    match posts {
        Some(posts) => {
            let Some(user) = conn
                .get_versia_user(&uuid, &EntityOrigin::Local(&state.instance_domain))
                .await
            else {
                return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
            };

            let collection = Collection::new(
                posts,
                count,
                state.outbox_pagnation_size,
                page,
                Some(user.uri),
                &state.instance_domain,
                &format!("versia/users/{uuid}/outbox"),
            );
            Ok(HttpResponse::Ok()
                .content_type("application/json; charset=UTF-8")
                .body(serde_json::to_string(&collection).unwrap()))
        }
        None => Err(ErrorNotFound(r#"{"error":"Not Found"}"#)),
    }
}
