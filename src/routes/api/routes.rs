use super::{login::login, signup::signup};

pub fn get_api_routes() -> actix_web::Scope {
    actix_web::web::scope("/api/bayou_v1")
        .service(signup)
        .service(login)
}
