use crate::entity::notification::NotificationContent;
use crate::entity::notification_log::NotificationLog;
use crate::{common::bson::*, entity::notification::Notification};
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use super::notification_dto::NotificationDTO;
use super::thread_dto::ThreadDTO;
use super::user_dto::UserDTO;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NotificationLogDTO {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub _id: Option<ObjectId>,
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub notification_id: Option<ObjectId>,
    pub notification: Option<NotificationDTO>,
    #[serde(
        rename = "ref_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub ref_id: Option<ObjectId>,
    pub thread: Option<ThreadDTO>,
    #[serde(
        rename = "user_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub user_id: Option<ObjectId>,
    pub user: Option<UserDTO>,
    pub is_read: bool,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
}

impl Into<NotificationLogDTO> for NotificationLog {
    fn into(self) -> NotificationLogDTO {
        NotificationLogDTO {
            _id: self._id,
            notification_id: self.notification_id,
            notification: None,
            ref_id: self.ref_id,
            user_id: self.user_id,
            user: None,
            is_read: self.is_read,
            created_at: self.created_at,
            updated_at: self.updated_at,
            thread: None,
        }
    }
}
