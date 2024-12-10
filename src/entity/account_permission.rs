use crate::dto::account_permission_dto::AccountPermissionDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPermission {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub account_id: Option<ObjectId>,
    pub permission_id: Option<ObjectId>,
    pub name: String,
    pub value: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}
impl AccountPermission {
    pub fn to_dto(self) -> AccountPermissionDTO {
        AccountPermissionDTO {
            id: self.id,
            account_id: self.account_id,
            permission_id: self.permission_id,
            name: self.name,
            value: self.value,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
