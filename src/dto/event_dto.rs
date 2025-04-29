use crate::common::bson::*;
use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{
    dto::{event_config_dto::EventConfigDTO, venue_location_dto::VenueLocationDTO},
    entity::{event_image::EventImage, theme::Theme},
};

use super::user_dto::UserDTO;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventDTO {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "host_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub host_id: Option<ObjectId>,
    pub host: Option<UserDTO>,
    #[serde(
        rename = "image_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub image_id: Option<ObjectId>,
    pub image: Option<EventImage>,
    #[serde(
        rename = "theme_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub theme_id: Option<ObjectId>,
    pub theme: Option<Theme>,
    pub invitation_id: String,
    pub event_name: String,
    pub event_description: String,
    pub status: String,
    pub datetime: Option<DateTime>,
    pub venue_location: Option<VenueLocationDTO>,
    pub config: EventConfigDTO,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub updated_at: DateTime,
}
