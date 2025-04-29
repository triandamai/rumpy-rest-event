use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::dto::event_dto::EventDTO;
use crate::dto::user_dto::UserDTO;
use crate::entity::event_invitation::EventInvitation;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventInvitationDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,
    pub user:Option<UserDTO>,
    #[serde(rename = "event_id", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<ObjectId>,
    pub event:Option<EventDTO>,
    pub invitation_type: String,
    pub invitation_code:String,
    pub expires_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Into<EventInvitationDTO> for EventInvitation{
    fn into(self) -> EventInvitationDTO {
        EventInvitationDTO{
            id: self.id,
            user_id: self.user_id,
            user: None,
            event_id: self.event_id,
            event: None,
            invitation_type: self.invitation_type,
            invitation_code: self.invitation_code,
            expires_at: self.expires_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}