use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Deserializer, Serializer};

// Custom serializer to convert ObjectId to string
pub fn serialize_object_id<S>(object_id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
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