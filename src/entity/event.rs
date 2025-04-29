use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::dto::{event_config_dto::EventConfigDTO, venue_location_dto::VenueLocationDTO};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "host_id")]
    pub host_id: Option<ObjectId>,
    #[serde(rename = "image_id")]
    pub image_id: Option<ObjectId>,
    #[serde(rename = "theme_id")]
    pub theme_id: Option<ObjectId>,
    pub invitation_id: String,
    pub event_name: String,
    pub event_description: String,
    pub datetime: Option<DateTime>,
    pub venue_location: Option<VenueLocationDTO>,
    pub status:String,
    pub config: EventConfigDTO,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
