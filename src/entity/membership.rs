use crate::dto::membership_dto::MembershipDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Membership {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub branch_id: Option<ObjectId>,
    pub name: String,
    pub price: f64,
    pub price_per_item: f64,
    pub quota: i64,
    pub description: String,
    pub kind:Option<String>,
    pub created_by_id: Option<ObjectId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted: bool,
}
impl Membership {
    pub fn to_dto(self) -> MembershipDTO {
        MembershipDTO {
            id: self.id,
            branch_id: self.branch_id,
            branch: None,
            name: self.name,
            price: self.price,
            quota: self.quota,
            price_per_item: self.price_per_item,
            description: self.description,
            created_by_id: self.created_by_id,
            created_by: None,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted: self.deleted,
        }
    }
}
