use bson::oid::ObjectId;
use bson::DateTime;
use log::info;
use serde::{Deserialize, Deserializer, Serializer};

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
