use super::{
    ap_api::routes::get_ap_routes,
    versia_api::{instance_discovery::versia_metadata, routes::get_versia_routes},
    webfinger::webfinger,
};

pub fn get_routes() -> actix_web::Scope {
    actix_web::web::scope("")
        .service(webfinger)
        .service(versia_metadata)
        .service(get_versia_routes())
        .service(get_ap_routes())
}
