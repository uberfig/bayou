use super::{
    inbox::{versia_shared_inbox, versia_user_inbox},
    outbox::versia_outbox,
    posts::versia_posts,
    users::versia_user,
};

pub fn get_versia_routes() -> actix_web::Scope {
    actix_web::web::scope("/versia")
        .service(versia_user_inbox)
        .service(versia_shared_inbox)
        .service(versia_posts)
        .service(versia_outbox)
        .service(versia_user)
}
