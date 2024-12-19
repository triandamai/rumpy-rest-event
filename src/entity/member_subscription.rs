use crate::dto::member_subscription_dto::MemberSubscriptionDTO;
use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberSubscription {
    pub id: Option<ObjectId>,
    pub member_id: Option<ObjectId>,
    pub membership_id: Option<ObjectId>,
    pub amount: f64,
    pub quota: i64,
    pub create_at: DateTime,
    pub update_at: DateTime,
}

impl MemberSubscription {
    pub fn to_dto(self) -> MemberSubscriptionDTO {
        MemberSubscriptionDTO {
            id: self.id,
            member_id: self.member_id,
            membership_id: self.membership_id,
            membership: None,
            quota: self.quota,
            amount: self.amount,
            create_at: self.create_at,
            update_at: self.update_at,
        }
    }
}
