use crate::dto::permission_dto::PermissionDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub value: String,
    pub name: String,
    pub group: String,
    pub description: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}

impl Permission {
    pub fn to_dto(self) -> PermissionDTO {
        PermissionDTO {
            id: self.id,
            value: self.value,
            name: self.name,
            group: self.group,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
