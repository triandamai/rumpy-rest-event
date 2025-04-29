use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VenueLocationDTO {
    pub map_id:Option<String>,
    pub map_details:Option<serde_json::Value>,
    pub venue_name:Option<String>,
    pub venue_address:Option<String>,
    pub venue_detail:Option<String>,
    pub lat:Option<i64>,
    pub lng:Option<i64>
}