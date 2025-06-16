//! `post /api/bayou_v1/community/create`
//!
//! create a new community, expects a [`crate::db::types::comm::community::Communityinfo`] and
//! a token in the header
//! - ok (200) community successfully created and a [`crate::routes::api::types::api_community::ApiCommunity`]
//! should be present in the body
//! - unauthorized (401) included token is not valid, community not created

use actix_web::{
    post,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};

use crate::{
    db::{pg_conn::PgConn, types::comm::community::Communityinfo},
    routes::api::{types::api_community::ApiCommunity, utilities::auth_header::get_auth_header},
};

#[post("/create")]
pub async fn create(
    req: HttpRequest,
    conn: Data<PgConn>,
    new_community: web::Json<Communityinfo>,
) -> Result<HttpResponse> {
    let Some(token) = get_auth_header(&req) else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body("invalid or missing auth header"));
    };
    if conn.validate_auth_token(&token).await.is_err() {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    }

    let Some(user) = conn.get_user_uid(&token.uid).await else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    };
    let community: ApiCommunity = conn
        .create_community(new_community.clone(), &user)
        .await
        .into();

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&community).expect("failed to serialize dbcommunity")))
}
