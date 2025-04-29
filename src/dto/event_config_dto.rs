use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventConfigDTO {
    pub max_capacity: i64,
    pub cost: f64,
    pub cost_type: String,
    pub cost_currency: String,
    pub cost_suggested_amount: f64,
    pub approval_required: bool,
    pub auto_reminder: bool,
    pub show_timestamp_activity: bool,
    pub show_guest_name: bool,
    pub show_guest_count: bool,
    pub event_password: Option<String>,
    pub allow_participant_album: bool,
    pub visibility: String,
}
