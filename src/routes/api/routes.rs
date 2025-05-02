use super::{
    community::routes::get_community_routes, login::login, message::routes::get_message_routes, regester_device::register_device, room::routes::get_room_routes, signup::signup
};

pub fn get_api_routes() -> actix_web::Scope {
    actix_web::web::scope("/api/bayou_v1")
        .service(signup)
        .service(login)
        .service(register_device)
        .service(get_community_routes())
        .service(get_message_routes())
        .service(get_room_routes())
}
