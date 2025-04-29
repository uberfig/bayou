use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Result,
};

use crate::{
    db::{pg_conn::PgConn, types::comm::community::Communityinfo},
    routes::api::types::info_with_token::BearrerWithInfo,
};

#[post("/create")]
async fn create(
    conn: Data<PgConn>,
    new_community: web::Json<BearrerWithInfo<Communityinfo>>,
) -> Result<HttpResponse> {
    if conn
        .validate_auth_token(&new_community.token)
        .await
        .is_err()
    {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    }
    let Some(user) = conn.get_user_uid(&new_community.token.uid).await else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    };
    let community = conn
        .create_community(new_community.info.clone(), &user)
        .await;

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&community).expect("failed to serialize dbcommunity")))
}
