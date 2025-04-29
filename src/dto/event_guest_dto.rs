use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::dto::event_dto::EventDTO;
use crate::dto::user_dto::UserDTO;
use crate::entity::event_guest::EventGuest;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventGuestDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,
    pub user:Option<UserDTO>,
    #[serde(rename = "event_id", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<ObjectId>,
    pub event:Option<EventDTO>,
    pub role: String,
    pub rsvp:Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}


impl Into<EventGuestDTO> for EventGuest {
    fn into(self) -> EventGuestDTO {
        EventGuestDTO{
            id: self.id,
            user_id: self.user_id,
            user: None,
            event_id: self.event_id,
            event: None,
            role: self.role,
            rsvp: self.rsvp,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}