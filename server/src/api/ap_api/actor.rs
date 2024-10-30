use actix_web::{
    error::{ErrorNotFound, ErrorUnauthorized},
    get,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};
use bayou_protocol::{
    protocol::ap_protocol::verification::verify_get,
    types::activitystream_objects::{actors::Actor, context::ContextWrap},
};

use crate::{
    api::headers::ActixHeaders,
    db::{
        conn::{Conn, EntityOrigin},
        dbconn::DbConn,
        utility::{instance_actor::InstanceActor, new_actor::NewLocal},
    },
};

#[get("/users/{preferred_username}")]
pub async fn get_actor(
    path: web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    dbg!(&request);
    let preferred_username = path.into_inner();

    if state.force_auth_fetch {
        let headers = ActixHeaders {
            headermap: request.headers().clone(),
        };
        let instance_key = conn.get_instance_actor(state.signing_algo).await;
        let verified = verify_get(
            &headers,
            request.path(),
            &state.instance_domain,
            &InstanceActor::get_key_id(&state.instance_domain),
            &mut instance_key.get_private_key(),
            instance_key.algorithm,
        )
        .await;

        if let Err(err) = verified {
            dbg!(&err);
            return Err(ErrorUnauthorized(serde_json::to_string(&err).unwrap()));
        }
    }

    let actor: Option<Actor> = conn
        .get_actor(
            &preferred_username,
            &EntityOrigin::Local(&state.instance_domain),
        )
        .await;

    let Some(actor) = actor else {
        return Err(ErrorNotFound(r#"{"error":"Not Found"}"#));
    };
    // let actor = actor.to_activitystream();
    let actor = actor.wrap_context();

    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(serde_json::to_string(&actor).unwrap()))
}

#[get("/create_test/{preferred_username}")]
pub async fn create_test(
    path: web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    let preferred_username = path.into_inner();

    let x = conn
        .create_user(
            &state.instance_domain,
            &NewLocal::new(
                preferred_username,
                "filler".to_string(),
                None,
                None,
                state.signing_algo,
            ),
        )
        .await
        .unwrap();

    Ok(HttpResponse::Ok().body(format!("{x}")))
}

#[get("/actor")]
pub async fn get_instance_actor(
    conn: Data<DbConn>,
    state: Data<crate::config::Config>,
) -> Result<HttpResponse> {
    println!("getting the instance actor");
    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(
            serde_json::to_string(
                &conn
                    .get_instance_actor(state.signing_algo)
                    .await
                    .to_actor(&state.instance_domain)
                    .wrap_context(),
            )
            .unwrap(),
        ))
}
