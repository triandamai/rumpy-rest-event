use std::fs;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use axum::extract::Multipart;
use chrono::Utc;
use headers::ContentType;
use log::info;
use mime::Mime;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::utils::get_extension_from_mime;
use std::path::{Path as FilePath};

#[derive(Deserialize, Serialize, Debug,Validate)]
pub struct UploadFileAttachment {
    #[validate(length(min = 2))]
    pub ref_id: String,
    #[validate(length(min = 2))]
    pub filename: String,
    #[validate(length(min = 2))]
    pub mime_type: String,
    #[validate(length(min = 2))]
    pub extension: String,
    #[validate(length(min = 2))]
    pub temp_path:String
}

impl UploadFileAttachment {
    pub async fn extract_multipart(mut multipart: Multipart) -> UploadFileAttachment {
        let mut metadata = UploadFileAttachment {
            ref_id: "".to_string(),
            filename: "".to_string(),
            mime_type: "".to_string(),
            extension: "".to_string(),
            temp_path: "".to_string(),
        };
        // Process each part of the multipart form data
        while let Some(field) = multipart.next_field().await.unwrap() {
            let field_name = field.name().map(|name| name.to_string());

            match field_name.as_deref() {
                Some("ref_id") => {
                    // Process the text field (username)
                    metadata.ref_id = field.text().await.unwrap();
                    info!(target:"extract_multipart", "found field :{} ",metadata.ref_id.clone());
                }
                Some("file") => {
                    // Process the file field (file)
                    let original_file_name = field.file_name().unwrap().to_string();
                    let mime_type = field.content_type().map(|mime| mime.clone());
                    let mime_type = mime_type.unwrap_or("image/png");
                    let mime_type = Mime::from_str(mime_type).unwrap_or(Mime::from(ContentType::png()));
                    let ext = get_extension_from_mime(&mime_type).unwrap_or(".png");
                    let original_file_name = original_file_name.replace(format!(".{}",ext.clone()).as_str(),"");

                    let current_time = Utc::now().timestamp();
                    let final_filename = format!("{}-{}.{}", original_file_name, current_time,ext.clone());
                    let location = format!("uploads/{}", final_filename);
                    // Read the file contents
                    let data = field.bytes().await.unwrap();

                    // Save the file to the "uploads" folder
                    let mut file = File::create(format!("uploads/{}", final_filename)).unwrap();
                    file.write_all(&data).unwrap();

                    metadata.filename = final_filename;
                    metadata.mime_type = mime_type.to_string();
                    metadata.extension = ext.to_string();
                    metadata.temp_path = location.clone();
                    info!(target:"extract_multipart", "temporary file place :{} ",location);
                }
                _ => {
                    info!(target: "extract_multipart","Unknown field encountered.");
                }
            }

        }
        metadata
    }

    pub fn remove_file(&self)->Result<String,String>{
        if FilePath::new(self.temp_path.clone().as_str()).exists() {
            let _remove = fs::remove_file(self.temp_path.clone().as_str());;
            info!(target:"remove file after used","File '{}' was removed successfully.", self.temp_path.clone());
        } else {
            info!(target:"remove file after used, failed","File '{}' does not exist.", self.temp_path.clone());
        }
        Ok("file was removed successfully.".to_string())
    }
}