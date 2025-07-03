use crate::routes::api::files::file_upload::upload_file;

pub fn get_file_routes() -> actix_web::Scope {
    actix_web::web::scope("/files").service(upload_file)
}
