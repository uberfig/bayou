use super::{
    actor::{get_actor, get_instance_actor},
    inbox::{private_inbox, shared_inbox},
    object::{get_object, get_object_create},
    outbox::{ap_outbox, create_ap_post},
};

pub fn get_ap_routes() -> actix_web::Scope {
    actix_web::web::scope("/ap")
        .service(get_actor)
        .service(get_instance_actor)
        .service(private_inbox)
        .service(shared_inbox)
        .service(get_object)
        .service(get_object_create)
        .service(create_ap_post)
        .service(ap_outbox)
}
