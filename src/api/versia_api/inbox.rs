use crate::api::headers::ActixHeaders;
use crate::db::conn::EntityOrigin;
use crate::db::dbconn::DbConn;
use actix_web::{error::ErrorBadRequest, http::StatusCode, rt::spawn};

use actix_web::{error::ErrorUnauthorized, post, web::Data, HttpRequest, HttpResponse, Result};
use bayou_protocol::cryptography::digest::sha256_hash;
use bayou_protocol::protocol::http_method::HttpMethod;
use bayou_protocol::protocol::versia_protocol::requests::Signer;
use bayou_protocol::protocol::versia_protocol::verify::verify_request;
use bayou_protocol::types::versia_types::entities::change_follow::ChangeFollowing;
use bayou_protocol::types::versia_types::entities::delete::{Delete, DeletedType};
use bayou_protocol::types::versia_types::entities::follow_response::FollowResponse;
use bayou_protocol::types::versia_types::entities::instance_metadata::InstanceMetadata;
use bayou_protocol::types::versia_types::entities::user::User;
use bayou_protocol::types::versia_types::postable::VersiaPostable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum VersiaInboxItem {
    Post(VersiaPostable),
    Delete(Delete),
    ChangeFollowing(ChangeFollowing),
    FollowResponse(FollowResponse),
    /// used when a user updates their profile
    User(Box<User>),
    InstanceMetadata(Box<InstanceMetadata>),
}

#[post("/users/{uuid}/inbox")]
pub async fn versia_user_inbox(
    request: HttpRequest,
    body: actix_web::web::Bytes,
    // actix_path: actix_web::web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    inbox(request, body, state, conn).await
}
#[post("/inbox")]
pub async fn versia_shared_inbox(
    request: HttpRequest,
    body: actix_web::web::Bytes,
    // actix_path: actix_web::web::Path<String>,
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    inbox(request, body, state, conn).await
}

pub async fn inbox(
    request: HttpRequest,
    body: actix_web::web::Bytes,
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) -> Result<HttpResponse> {
    let path = request.path();

    let Ok(body) = String::from_utf8(body.to_vec()) else {
        return Err(ErrorUnauthorized("bad request body"));
    };
    let hash = sha256_hash(body.as_bytes());

    let headers = ActixHeaders {
        headermap: request.headers().clone(),
    };

    let authorized = verify_request(&headers, HttpMethod::Get, &path, &hash, &**conn).await;

    let signer = match authorized {
        Ok(x) => x,
        Err(err) => return Err(ErrorUnauthorized(err)),
    };

    let deserialized: Result<VersiaInboxItem, _> = serde_json::from_str(&body);

    match deserialized {
        Ok(x) => {
            spawn(handle_inbox(signer, x, state, conn));
            Ok(HttpResponse::Ok()
                .status(StatusCode::ACCEPTED)
                .content_type("application/json; charset=UTF-8")
                .body(""))
        }
        Err(x) => Err(ErrorBadRequest(x)),
    }
}

#[allow(unused_variables)]
pub async fn handle_inbox(
    signer: Signer,
    entity: VersiaInboxItem,
    state: Data<crate::config::Config>,
    conn: Data<DbConn>,
) {
    // all signers should have a domain. federation with an ip address will
    // never be supported as they can 1. be dynamic, 2. be used to skirt defeds
    let Some(authoratative_domain) = signer.domain() else {
        return;
    };
    match entity {
        VersiaInboxItem::Post(postable) => {
            // another instance is trying to impersonate this user
            // we could log this in the future
            if postable.get_author().domain().ne(&signer.domain()) {
                return;
            }
            let post = conn
                .create_versia_post(postable, &EntityOrigin::Federated(authoratative_domain))
                .await
                .expect("failed to insert post");
        }
        VersiaInboxItem::Delete(delete) => match delete.deleted_type {
            DeletedType::Note | DeletedType::Share => {
                conn.delete_post(&delete.id, &EntityOrigin::Federated(authoratative_domain))
                    .await
                    .expect("failed to delete post");
            }
            DeletedType::User => todo!(),
        },
        VersiaInboxItem::ChangeFollowing(change_following) => todo!(),
        VersiaInboxItem::FollowResponse(follow_response) => todo!(),
        VersiaInboxItem::User(user) => todo!(),
        VersiaInboxItem::InstanceMetadata(instance_metadata) => todo!(),
    }
}
