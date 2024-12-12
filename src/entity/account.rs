use crate::dto::account_dto::AccountDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub gender: String,
    pub job_title: String,
    pub report_to: Option<ObjectId>,
    pub branch_id: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}

impl Account {
    pub fn to_dto(self) -> AccountDTO {
        AccountDTO {
            id: self.id,
            full_name: self.full_name,
            email: self.email,
            gender: self.gender,
            job_title: self.job_title,
            report_to: self.report_to,
            branch_id: self.branch_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
