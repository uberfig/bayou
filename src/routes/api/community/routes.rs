use super::{
    create::create, create_room::create_room, get_members::get_members, get_rooms::get_rooms,
};

pub fn get_community_routes() -> actix_web::Scope {
    actix_web::web::scope("/community")
        .service(create)
        .service(create_room)
        .service(get_rooms)
        .service(get_members)
}
