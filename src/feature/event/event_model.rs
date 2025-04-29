use bson::DateTime;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateNewEventRequest {
    pub title: String,
}


#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateEventRequest {
    pub theme_id: Option<ObjectId>,
    pub event_name: Option<String>,
    pub event_description: Option<String>,
    pub status: Option<String>,
    pub datetime: Option<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateEventConfigRequest {
    pub max_capacity: Option<i64>,
    pub cost: Option<f64>,
    pub cost_type: Option<String>,
    pub cost_currency: Option<String>,
    pub cost_suggested_amount: Option<f64>,
    pub approval_required: Option<bool>,
    pub auto_reminder: Option<bool>,
    pub show_timestamp_activity: Option<bool>,
    pub show_guest_name: Option<bool>,
    pub show_guest_count: Option<bool>,
    pub event_password: Option<String>,
    pub allow_participant_album: Option<bool>,
    pub visibility: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone,Validate)]
pub struct UpdateEventLocationRequest {
    pub map_id:Option<String>,
    pub map_details:Option<serde_json::Value>,
    pub venue_name:Option<String>,
    pub venue_address:Option<String>,
    pub venue_detail:Option<String>,
    pub lat:Option<i64>,
    pub lng:Option<i64>
}

#[derive(Serialize, Deserialize, Debug, Clone,Validate)]
pub struct UpdateEventHostRequest {
    pub event_id:String,
    pub user_id:String,
    pub role:String,
}