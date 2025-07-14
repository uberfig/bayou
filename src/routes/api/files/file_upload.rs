//! `/api/bayou_v1/files/new`

use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};
use actix_web::{post, web, web::Data, HttpRequest, HttpResponse, Result};
use serde::Deserialize;

use crate::{db::pg_conn::PgConn, routes::api::utilities::auth_header::get_auth_header};

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    // #[multipart(limit = "100MB")]
    file: TempFile,
    // json: MPJson<Metadata>,
}

#[post("/new")]
pub async fn upload_file(
    state: Data<crate::config::Config>,
    req: HttpRequest,
    conn: Data<PgConn>,
    MultipartForm(form): MultipartForm<UploadForm>
) -> Result<HttpResponse> {
    println!(
        "Uploaded file {}, with size: {}, with mime: {}",
        // form.json.name, 
        form.file.file_name.unwrap_or("[no name provided]".to_string()),
        form.file.size,
        form.file.content_type.map(|x| x.essence_str().to_string()).unwrap_or("missing mime type".to_string())
    );
    
    let Some(token) = get_auth_header(&req) else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body("invalid or missing auth header"));
    };
    if conn.validate_auth_token(&token).await.is_err() {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .body(""));
    }
    
    Ok(HttpResponse::Ok().body(""))
}