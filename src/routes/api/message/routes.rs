use super::send_message::send_message;

pub fn get_message_routes() -> actix_web::Scope {
    actix_web::web::scope("/message")
        .service(send_message)
}
