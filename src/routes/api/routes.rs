use super::signup::signup;

pub fn get_api_routes() -> actix_web::Scope {
    actix_web::web::scope("/api").service(signup)
}
