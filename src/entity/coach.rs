use crate::dto::coach_dto::CoachDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Coach {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub branch_id: Option<ObjectId>,
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
    pub gender: String,
    pub created_by_id: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted:bool,
}

impl Coach {
    pub fn to_dto(self) -> CoachDTO {
        CoachDTO {
            id: self.id,
            branch_id: self.branch_id,
            full_name: self.full_name,
            email: self.email,
            phone_number: self.phone_number,
            gender: self.gender,
            created_by_id: self.created_by_id,
            created_by: None,
            profile_picture: None,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
