use crate::common::utils::{
    get_extension_from_mime, validate_multi_extractor, validate_single_extractor,
};
use axum::extract::multipart::Field;
use axum::extract::Multipart;
use bson::oid::ObjectId;
use chrono::Utc;
use headers::ContentType;
use log::info;
use mime::Mime;
use s3::serde_types::Object;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path as FilePath;
use std::str::FromStr;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct FileTemp {
    #[validate(length(min = 1))]
    pub ref_id: String,
    #[validate(length(min = 1))]
    pub filename: String,
    #[validate(length(min = 1))]
    pub mime_type: String,
    #[validate(length(min = 1))]
    pub extension: String,
    #[validate(length(min = 1))]
    pub temp_path: String,
}
impl FileTemp {
    pub fn to_dto(self) -> FileTemp {
        FileTemp {
            ref_id: self.ref_id,
            filename: self.filename,
            mime_type: self.mime_type,
            extension: self.extension,
            temp_path: self.temp_path,
        }
    }

    pub fn validate_file(&self) -> Result<bool, ValidationErrors> {
        let validate = self.validate();
        if validate.is_err() {
            return Err(validate.unwrap_err());
        }
        Ok(validate.is_ok())
    }

    pub fn remove_file(&self) -> Result<String, String> {
        remove_temp_file(self.temp_path.as_str())
    }
}
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct MultiFileExtractor {
    #[validate(length(min = 1))]
    pub ref_id: String,
    pub temp_file: HashMap<String, FileTemp>,
    pub notes: String,
    pub is_error: bool,
    pub error_message: Option<String>,
}

impl MultiFileExtractor {
    pub async fn extract(mut multipart: Multipart) -> MultiFileExtractor {
        let mut temporary = HashMap::<String, FileTemp>::new();
        let mut ref_id = String::new();
        let mut notes = String::new();
        let mut is_error = false;
        let mut error_message: Option<String> = None;
        // Process each part of the multipart form data
        while let Ok(Some(field)) = multipart.next_field().await {
            let field_name = field.name().map(|name| name.to_string());
            let field_type = field
                .content_type()
                .map(|mime| mime.to_string())
                .unwrap_or(String::from("text/*"));

            match field_name.as_deref() {
                Some("ref_id") => {
                    // Process the text field (username)
                    ref_id = field.text().await.unwrap();
                    info!(target:"extract_multipart", "found field :{} ",ref_id.clone());
                }
                Some("notes") => {
                    // Process the text field (username)
                    notes = field.text().await.unwrap();
                    info!(target:"extract_multipart", "found field :{} ",ref_id.clone());
                }
                name => {
                    if field_type.starts_with("image/") {
                        let file = field_to_temp_file(&ref_id, field).await;
                        if file.is_ok() {
                            is_error = false;
                            temporary.insert(name.unwrap_or("").to_string(), file.unwrap());
                        } else {
                            is_error = true;
                            error_message = Some("Limit file has react".to_string());
                        }
                    } else {
                        info!(target: "extract_multipart","Unknown field encountered {}.",field_type);
                    }
                }
            }
        }

        MultiFileExtractor {
            ref_id,
            temp_file: temporary,
            is_error: is_error,
            notes: notes,
            error_message: error_message,
        }
    }

    pub fn remove_file(&self) -> Result<String, String> {
        self.temp_file.iter().for_each(|(_, value)| {
            let _ok = value.remove_file();
        });
        Ok("Success clearing temporary file.".to_string())
    }
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct SingleFileExtractor {
    #[validate(length(min = 1))]
    pub ref_id: String,
    pub multipart_temp_file: Option<FileTemp>,
    pub is_error: bool,
    pub error_message: Option<String>,
}

impl SingleFileExtractor {
    pub fn to_dto(self) -> SingleFileExtractor {
        SingleFileExtractor {
            ref_id: self.ref_id,
            multipart_temp_file: self.multipart_temp_file,
            is_error: self.is_error,
            error_message: self.error_message,
        }
    }

    pub fn validate_body(&self) -> Result<bool, ValidationErrors> {
        let body = self.validate();
        if body.is_err() {
            return Err(body.unwrap_err());
        }

        match &self.multipart_temp_file {
            None => {
                let mut errors = ValidationErrors::new();
                let _add = errors.add("file", ValidationError::new("file cannot empty"));
                Err(errors)
            }
            Some(file) => file.validate_file(),
        }
    }

    pub async fn extract(mut multipart: Multipart) -> SingleFileExtractor {
        let mut metadata = SingleFileExtractor {
            ref_id: "".to_string(),
            multipart_temp_file: None,
            is_error: false,
            error_message: None,
        };
        let mut file_find = false;
        // Process each part of the multipart form data

        while let Ok(Some(field)) = multipart.next_field().await {
            let field_name = field.name().map(|name| name.to_string());

            match field_name.as_deref() {
                Some("ref_id") => {
                    // Process the text field (username)
                    metadata.ref_id = field.text().await.unwrap();
                    info!(target:"extract_multipart", "found field :{} ",metadata.ref_id.clone());
                }
                Some("file") => {
                    //make sure receive only one file
                    if file_find {
                        info!(target:"extract_multipart","file already exist skip to next");
                        continue;
                    }
                    file_find = true;
                    let file = field_to_temp_file(&metadata.ref_id.clone(), field).await;
                    if file.is_ok() {
                        metadata.multipart_temp_file = Some(file.unwrap());
                    } else {
                        metadata.is_error = true;
                        metadata.error_message = Some(file.unwrap_err())
                    }
                }
                _ => {
                    info!(target: "extract_multipart","Unknown field encountered.");
                }
            }
        }
        metadata
    }

    pub fn remove_file(&self) -> Result<String, String> {
        if let Some(temp_file) = &self.multipart_temp_file {
            return remove_temp_file(temp_file.temp_path.as_str());
        }
        Ok("Success clearing temporary file.".to_string())
    }

    pub fn file(&self) -> FileTemp {
        self.multipart_temp_file.clone().unwrap()
    }
}

pub async fn field_to_temp_file(ref_id: &String, field: Field<'_>) -> Result<FileTemp, String> {
    //make sure receive only one file
    // Process the file field (file)
    let mime_type = field.content_type().map(|mime| mime);
    let mime_type = mime_type.unwrap_or("image/png");
    let mime_type = Mime::from_str(mime_type).unwrap_or(Mime::from(ContentType::png()));
    let ext = get_extension_from_mime(&mime_type).unwrap_or(".png");

    let object_id = ObjectId::new().to_string();
    let final_filename = format!("{}.{}", object_id, ext);
    let location = format!("uploads/{}", final_filename);
    // Read the file contents
    let data = field.bytes().await;
    if data.is_err() {
        return Err(data.unwrap_err().body_text());
    }

    // Save the file to the "uploads" folder
    let mut file = File::create(format!("uploads/{}", final_filename)).unwrap();
    file.write_all(&data.unwrap()).unwrap();

    let temp_file = FileTemp {
        ref_id: ref_id.clone(),
        filename: final_filename,
        mime_type: mime_type.to_string(),
        extension: ext.to_string(),
        temp_path: location.clone(),
    };
    info!(target:"extract_multipart", "temporary file place :{} ",location);
    Ok(temp_file)
}

pub fn remove_temp_file(path: &str) -> Result<String, String> {
    if FilePath::new(path).exists() {
        let _remove = fs::remove_file(path);
        info!(target:"remove file after used","File '{}' was removed successfully.", path);
    } else {
        info!(target:"remove file after used, failed","File '{}' does not exist.", path);
    }
    Ok("file was removed successfully.".to_string())
}
