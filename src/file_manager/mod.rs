use std::{fs::File, io::{Read, Write}, str::FromStr};

use actix_multipart::form::tempfile::TempFile;
use actix_web::web::Data;
use mime::Mime;
use mime2ext::mime2ext;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::pg_conn::PgConn;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "type", content = "args")]
pub enum FileManager {
    Local {
        base_path: String
    }
}

impl FileManager {
    pub async fn create_user_file(&self, conn: Data<PgConn>, file: TempFile, owner: Uuid, description: Option<String>) -> Result<Uuid, ()> {
        let content_type = file.content_type.clone().unwrap_or(Mime::from_str("application/octet-stream").unwrap());
        
        let file_id = conn.register_file(Some(owner), description, None, content_type.clone()).await;
        
        let extension = FileManager::get_extension(content_type.clone());
        let path = format!("{}/files/{file_id}/{file_id}{extension}", owner.as_simple().to_string());
        self.write_file(path, file)?;

        Ok(file_id)
    }
    fn get_extension(content_type: Mime) -> String {
        let extension = mime2ext(content_type);
        let extension = match extension {
            Some(x) => ".".to_owned()+x,
            None => "".to_owned(),
        };
        extension
    }
    fn write_file(&self, path: String, uploaded_file: TempFile) -> Result<(),()> {
        match self {
            FileManager::Local { base_path } => {
                let disk_path = format!("{base_path}/{path}");
                FileManager::write_file_local(disk_path, uploaded_file).map_err(|_| ())?;
                Ok(())
            },
        }
    }
    fn write_file_local(disk_path: String, mut uploaded_file: TempFile) -> std::io::Result<()> {
        let mut file = File::create(disk_path)?;
        let mut buffer = Vec::new();
        uploaded_file.file.read_to_end(&mut buffer)?;
        file.write_all(&buffer)?;
        file.flush()?;
        Ok(())
    }
}