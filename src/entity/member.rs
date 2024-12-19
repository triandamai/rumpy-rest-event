use crate::dto::member_dto::MemberDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub member_code: String,
    pub branch_id: Option<ObjectId>,
    pub membership_id: Option<ObjectId>,
    pub created_by_id: Option<ObjectId>,
    pub coach_id: Option<ObjectId>,
    pub full_name: String,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub identity_number: Option<String>,
    pub nfc_number: Option<String>,
    pub phone_number: Option<String>,
    pub is_member: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}

impl Member {
    pub fn to_dto(self) -> MemberDTO {
        MemberDTO {
            id: self.id,
            member_code: self.member_code,
            branch_id: self.branch_id,
            branch: None,
            created_by_id: self.created_by_id,
            created_by: None,
            coach_id: self.coach_id,
            coach: None,
            subscription: None,
            nfc_id:self.nfc_number,
            full_name: self.full_name,
            gender: self.gender,
            email: self.email,
            identity_number: self.identity_number,
            phone_number: self.phone_number,
            profile_picture: None,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
