use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 7))]
    pub current_password: String,
    #[validate(length(min = 7))]
    pub new_password: String,
}
