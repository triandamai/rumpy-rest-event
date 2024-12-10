use crate::dto::member_dto::MemberDTO;
use bson::oid::ObjectId;
use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use bson::DateTime;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub member_code: String,
    pub branch_id: Option<ObjectId>,
    pub created_by: Option<ObjectId>,
    pub coach_id: Option<ObjectId>,
    pub full_name: String,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
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
            created_by: self.created_by,
            coach_id: self.coach_id,
            full_name: self.full_name,
            gender: self.gender,
            email: self.email,
            date_of_birth: self.date_of_birth,
            phone_number: self.phone_number,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
