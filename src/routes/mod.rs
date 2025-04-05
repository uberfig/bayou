use api::routes::get_api_routes;

pub mod api;

pub fn get_routes() -> actix_web::Scope {
    actix_web::web::scope("")
        .service(get_api_routes())
}