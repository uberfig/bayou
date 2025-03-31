use super::webfinger::webfinger;

pub fn get_well_known_routes() -> actix_web::Scope {
    actix_web::web::scope("/.well-known").service(webfinger)
}
