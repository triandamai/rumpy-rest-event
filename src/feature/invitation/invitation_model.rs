use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateInvitationLinkRequest {
    pub expired_at: String,
    pub event_id: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct SendInvitationRequest {
    pub event_id: String,
    pub users: Vec<InvitationRequest>,
}
#[derive(Serialize, Deserialize, Validate)]
pub struct InvitationRequest {
    pub expired_at: String,
    pub user_id: String,
}
