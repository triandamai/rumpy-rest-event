use crate::dto::branch_dto::BranchDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub branch_name: String,
    pub branch_description: String,
    pub branch_email: Option<String>,
    pub branch_phone_number: Option<String>,
    pub branch_address: Option<String>,
    pub branch_owner: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted:bool,
}

impl Branch {
    pub fn to_dto(self) -> BranchDTO {
        BranchDTO {
            id: self.id,
            branch_name: self.branch_name,
            branch_description: self.branch_description,
            branch_email: self.branch_email,
            branch_phone_number: self.branch_phone_number,
            branch_address: self.branch_address,
            branch_owner: self.branch_owner,
            owner: None,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
