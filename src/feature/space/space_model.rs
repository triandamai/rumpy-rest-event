use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateSpaceRequest {
    pub name: String,
    pub space_thumbnail_id: Option<i32>,
    pub is_public: bool,
    pub description: String,
}
