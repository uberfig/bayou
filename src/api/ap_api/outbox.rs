use actix_web::{
    error::{ErrorNotFound, ErrorUnauthorized},
    get,
    http::Error,
    post,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};
use bayou_protocol::protocol::ap_protocol::verification::verify_get;

use crate::{
    api::{headers::ActixHeaders, page_query::Page},
    db::{
        conn::{Conn, EntityOrigin},
        utility::instance_actor::InstanceActor,
    },
};

#[get("/users/{preferred_username}/outbox")]
pub async fn ap_outbox(
    path: web::Path<String>,
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
    request: HttpRequest,
    page: actix_web::web::Query<Option<Page>>,
) -> Result<HttpResponse> {
    let preferred_username = path.into_inner();
    let path = request.path();
    let page = match page.into_inner() {
        Some(x) => x.page,
        None => 1,
    };
    if page.eq(&0) {
        return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
    }

    if state.force_auth_fetch {
        let headers = ActixHeaders {
            headermap: request.headers().clone(),
        };
        let instance_key = conn.get_instance_actor().await;
        let verified = verify_get(
            &headers,
            path,
            &state.instance_domain,
            &InstanceActor::get_key_id(&state.instance_domain),
            &mut instance_key.get_private_key(),
        )
        .await;

        if let Err(err) = verified {
            return Err(ErrorUnauthorized(serde_json::to_string(&err).unwrap()));
        }
    }

    let Some(count) = conn
        .get_user_post_count(
            &preferred_username,
            &EntityOrigin::Local(&state.instance_domain),
        )
        .await
    else {
        return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
    };

    let posts = conn
        .get_user_posts_ap(
            &preferred_username,
            &EntityOrigin::Local(&state.instance_domain),
            state.outbox_pagnation_size,
            page,
        )
        .await;

    match posts {
        Some(posts) => {
            let Some(user) = conn
                .get_actor(
                    &preferred_username,
                    &EntityOrigin::Local(&state.instance_domain),
                )
                .await
            else {
                return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
            };

            todo!()

            // let collection = Collection::new(
            //     posts,
            //     count,
            //     state.outbox_pagnation_size,
            //     page,
            //     Some(user.uri),
            //     &state.instance_domain,
            //     &format!("users/{uuid}/outbox/versia"),
            // );
            // Ok(HttpResponse::Ok()
            //     .content_type("application/json; charset=UTF-8")
            //     .body(serde_json::to_string(&collection).unwrap()))
        }
        None => Err(ErrorNotFound(r#"{"error":"Not Found"}"#)),
    }
}

#[post("/users/{preferred_username}/outbox")]
pub async fn create_ap_post(
    path: web::Path<String>,
    body: web::Bytes,
    conn: Data<Box<dyn Conn + Sync>>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse, Error> {
    todo!()
}
