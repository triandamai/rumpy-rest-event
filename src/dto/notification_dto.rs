use crate::entity::notification::NotificationContent;
use crate::{common::bson::*, entity::notification::Notification};
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NotificationDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    #[serde(rename = "ref_id", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<ObjectId>,
    pub kind: String,
    pub title: String,
    pub body: Option<NotificationContent>,
    pub notification_type: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
}

impl Into<NotificationDTO> for Notification {
    fn into(self) -> NotificationDTO {
        NotificationDTO {
            _id: self._id,
            ref_id: self.ref_id,
            kind: self.kind,
            title: self.title,
            body: self.body,
            notification_type: self.notification_type,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
