use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized, ErrorUnprocessableEntity},
    get, post,
    rt::spawn,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};
use bayou_protocol::{
    protocol::ap_protocol::verification::verify_get,
    types::activitystream_objects::new_post::NewPost,
};

use crate::{
    api::{headers::ActixHeaders, page_query::Page},
    db::{
        conn::{Conn, EntityOrigin},
        dbconn::DbConn,
        utility::instance_actor::InstanceActor,
    },
    tasks::notify_followers::notify_followers,
};

#[get("/users/{preferred_username}/outbox")]
pub async fn ap_outbox(
    path: web::Path<String>,
    conn: Data<DbConn>,
    state: Data<crate::config::Config>,
    request: HttpRequest,
    page: actix_web::web::Query<Page>,
) -> Result<HttpResponse> {
    let preferred_username = path.into_inner();
    let path = request.path();
    let page = page.into_inner();
    let is_page = page.is_page.unwrap_or(false);
    let page = page.page.unwrap_or(1);
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
            instance_key.algorithm,
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
    if !is_page {
        //the root ordered collection type
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=UTF-8")
            .body(serde_json::to_string("").unwrap()));
    }

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
    // path: web::Path<String>,
    body: web::Bytes,
    conn: Data<DbConn>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse> {
    //TODO impliment oauth logic
    let new_post: NewPost = match serde_json::from_slice(&body) {
        Ok(ok) => ok,
        Err(err) => return Err(ErrorUnprocessableEntity(err)),
    };
    match conn
        .new_local_post(new_post, &EntityOrigin::Local(&state.instance_domain))
        .await
    {
        Ok(ok) => {
            let taken = ok.clone();
            spawn(async move {
                notify_followers(conn, &taken, EntityOrigin::Local(&state.instance_domain)).await
            });
            Ok(HttpResponse::Created().body(ok))
        }
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}
