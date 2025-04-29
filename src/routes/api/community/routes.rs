use super::{create::create, create_room::create_room};

pub fn get_community_routes() -> actix_web::Scope {
    actix_web::web::scope("/community")
        .service(create)
        .service(create_room)
}
