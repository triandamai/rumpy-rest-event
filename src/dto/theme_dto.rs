use bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use crate::dto::event_dto::EventDTO;
use crate::dto::user_dto::UserDTO;
use crate::entity::theme::Theme;
use crate::common::bson::{deserialize_object_id,serialize_object_id,serialize_datetime};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThemeDTO {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub theme: String,
    #[serde(
        rename = "user_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub user_id: Option<ObjectId>,
    pub user: Option<UserDTO>,
    #[serde(
        rename = "event_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub event_id: Option<ObjectId>,
    pub event: Option<EventDTO>,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
}

impl Into<ThemeDTO> for Theme{
    fn into(self) -> ThemeDTO {
        ThemeDTO{
            id: self.id,
            theme: self.theme,
            user_id: self.user_id,
            user:None,
            event_id: self.event_id,
            event:None,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}