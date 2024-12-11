use crate::dto::member_log_dto::MemberLogDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberLog {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub member_id: Option<ObjectId>,
    pub created_by_id: Option<ObjectId>,
    pub name: String,
    pub value: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}

impl MemberLog {
    pub fn to_dto(self) -> MemberLogDTO {
        MemberLogDTO {
            id: self.id,
            member_id: self.member_id,
            created_by_id: self.created_by_id,
            name: self.name,
            value: self.value,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
