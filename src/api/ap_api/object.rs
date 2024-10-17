use actix_web::{
    error::{ErrorNotFound, ErrorUnauthorized},
    get,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};
use bayou_protocol::{
    protocol::ap_protocol::verification::verify_get,
    types::activitystream_objects::{
        create::{Create, CreateType},
        link::RangeLinkItem,
    },
};
use url::Url;

use crate::{
    api::headers::ActixHeaders,
    db::{conn::Conn, utility::instance_actor::InstanceActor},
};

#[get("/users/{preferred_username}/statuses/{id}")]
pub async fn get_object(
    path: web::Path<(String, String)>,
    conn: Data<Box<dyn Conn + Sync>>,
    request: HttpRequest,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse> {
    dbg!(&request);

    let (_preferred_username, id) = path.into_inner();
    let path = request.path();

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

    let object = conn
        .get_ap_post(
            &id,
            &crate::db::conn::EntityOrigin::Local(&state.instance_domain),
        )
        .await;

    match object {
        Some(object) => Ok(HttpResponse::Ok()
            .content_type("application/activity+json; charset=utf-8")
            .body(serde_json::to_string(&object.wrap_context()).unwrap())),
        None => Err(ErrorNotFound(r#"{"error":"Not Found"}"#)),
    }
}

#[get("/users/{preferred_username}/statuses/{id}/activity")]
pub async fn get_object_create(
    path: web::Path<(String, String)>,
    conn: Data<Box<dyn Conn + Sync>>,
    request: HttpRequest,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse> {
    dbg!(&request);

    let (_preferred_username, id) = path.into_inner();
    let path = request.path();

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

    let object = conn
        .get_ap_post(
            &id,
            &crate::db::conn::EntityOrigin::Local(&state.instance_domain),
        )
        .await;

    match object {
        Some(object) => {
            let activity = Create {
                type_field: CreateType::Create,
                id: Url::parse(&format!("https://{}{}", &state.instance_domain, path))
                    .expect("generated invalid url"),
                actor: object.actor().clone(),
                object: RangeLinkItem::Item(object),
            };

            Ok(HttpResponse::Ok()
                .content_type("application/activity+json; charset=utf-8")
                .body(serde_json::to_string(&activity.wrap_context()).unwrap()))
        }
        None => Err(ErrorNotFound(r#"{"error":"Not Found"}"#)),
    }
}
