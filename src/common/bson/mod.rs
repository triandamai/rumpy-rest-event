use bson::oid::ObjectId;
use bson::DateTime;
use log::info;
use serde::{Deserialize, Deserializer, Serializer};

use crate::dto::file_attachment_dto::FileAttachmentDTO;

use crate::common::env_config::EnvConfig;

use super::constant::{
    KIND_COACH_PROFILE_PICTURE, KIND_MEMBER_BODY_IMAGE, KIND_MEMBER_DATA_IMAGE,
    KIND_MEMBER_PROFILE_PICTURE, KIND_PRODUCT_IMAGE, KIND_USER_PROFILE_PICTURE,
};

// Custom serializer to convert ObjectId to string
pub fn serialize_object_id<S>(
    object_id: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if object_id.is_some() {
        serializer.serialize_str(&object_id.unwrap().to_hex()) // Convert ObjectId to a hex string
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize_object_id<'de, D>(deserializer: D) -> Result<Option<ObjectId>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = ObjectId::deserialize(deserializer);
    //String::deserialize(deserializer);
    // info!(target: "deserialize","{:?}",s);
    if s.is_err() {
        Ok(None)
    } else {
        Ok(Some(s?))
    }
}

//custom serializer for DateTime
pub fn serialize_datetime<S>(val: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = val.try_to_rfc3339_string();
    if formatted.is_ok() {
        serializer.serialize_str(formatted.unwrap().as_str()) // Convert ObjectId to a hex string
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = DateTime::deserialize(deserializer);
    //String::deserialize(deserializer);
    // info!(target: "deserialize","{:?}",s);
    if s.is_err() {
        Err(s.unwrap_err())
    } else {
        Ok(s?)
    }
}

pub fn non_empty<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let vec = <Vec<T>>::deserialize(d)?;
    if vec.is_empty() {
        Ok(vec![])
    } else {
        Ok(vec)
    }
}

//file attachment to path
pub fn serialize_file_attachment<S>(
    file: &Option<FileAttachmentDTO>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if file.is_some() {
        let file = transform_file_attachment(file.clone().unwrap());

        serializer.serialize_some(&file)
    } else {
        serializer.serialize_none()
    }
}

//file attachment to path
pub fn serialize_file_attachments<S>(
    file: &Option<Vec<FileAttachmentDTO>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(file1) = file {
        let transform = file1
            .iter()
            .map(|f| transform_file_attachment(f.clone()))
            .collect::<Vec<FileAttachmentDTO>>();

        serializer.serialize_some(&transform)
    } else {
        serializer.serialize_none()
    }
}

fn transform_file_attachment(mut file: FileAttachmentDTO) -> FileAttachmentDTO {
    let env_mode = std::env::var("MODE").unwrap_or("DEV".to_string());
    let base_url_key = format!("BASE_URL_{}", env_mode);
    let env_base_url = std::env::var(base_url_key.clone());
    let default = String::new();
    if env_base_url.is_err() {
        info!(target:"error get base url","{}",env_base_url.clone().unwrap_err());
    }
    let base_url = env_base_url.unwrap_or(default);

    let bucket = get_bucket_name(file.kind.clone());
    let url = format!("{}{}?file_name={}", base_url, bucket, file.filename);
    file.full_path = Some(url.to_string());
    return file;
}

fn get_bucket_name(kind: String) -> String {
    if kind.eq(KIND_USER_PROFILE_PICTURE) {
        "profile-picture".to_string()
    } else if kind.eq(KIND_PRODUCT_IMAGE) {
        "product-image".to_string()
    } else if kind.eq(KIND_COACH_PROFILE_PICTURE) {
        "coach-profile-picture".to_string()
    } else if kind.eq(KIND_MEMBER_PROFILE_PICTURE) {
        "member-profile-picture".to_string()
    } else if kind.eq(KIND_MEMBER_BODY_IMAGE) {
        "member-log".to_string()
    } else if kind.eq(KIND_MEMBER_DATA_IMAGE) {
        "member-log".to_string()
    } else {
        "none".to_string()
    }
}
