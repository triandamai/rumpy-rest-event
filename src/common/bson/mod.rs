use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Deserializer, Serializer};

use crate::dto::file_attachment_dto::FileAttachmentDTO;

use crate::common::env_config::EnvConfig;

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
        let config = EnvConfig::init();

        let mut file1 = file.clone().unwrap();
        let bucket = if file1.kind == "USER" {
            "profile-picture"
        } else if file1.kind == "PRODUCT" {
            "product-image"
        } else if file1.kind == "COACH" {
            "coach-profile-picture"
        } else if file1.kind == "MEMBER" {
            "member-profile-picture"
        } else {
            "none"
        };

        let url = format!("{}{}?file_name={}", config.base_url, bucket, file1.filename);
        file1.full_path = Some(url.to_string());

        serializer.serialize_some(&file1)
    } else {
        serializer.serialize_none()
    }
}
