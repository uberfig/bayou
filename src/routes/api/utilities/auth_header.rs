use actix_web::HttpRequest;

use crate::db::types::tokens::auth_token::AuthToken;

pub fn get_auth_header<'a>(req: &'a HttpRequest) -> Option<AuthToken> {
    let header = req.headers().get("authorization")?.to_str().ok();
    header.map(|heaher| {
        serde_json::from_str(heaher).ok()
    }).flatten()
}