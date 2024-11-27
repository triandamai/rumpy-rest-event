use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct AssignPermissionRequest {
    #[validate(length(min = 24,max=24))]
    pub user_id: String,
    #[validate(length(min = 24,max=24))]
    pub permission_id: String,
}
