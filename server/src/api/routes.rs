use super::{
    ap_api::routes::get_ap_routes,
    versia_api::{instance_discovery::versia_metadata, routes::get_versia_routes}, well_known::routes::get_well_known_routes,
};

pub fn get_routes() -> actix_web::Scope {
    actix_web::web::scope("")
        .service(versia_metadata)
        .service(get_versia_routes())
        .service(get_ap_routes())
        .service(get_well_known_routes())
}
