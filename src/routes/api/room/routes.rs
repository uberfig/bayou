use super::messages::get_messages;

pub fn get_room_routes() -> actix_web::Scope {
    actix_web::web::scope("/room").service(get_messages)
}
