use crate::dto::user_dto::UserDTO;
use crate::dto::user_profile_dto::UserProfileDTO;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 7))]
    pub current_password: String,
    #[validate(length(min = 7))]
    pub new_password: String,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserWithProfiledResponse {
    pub user:Option<UserDTO>,
    pub profile:Option<UserProfileDTO>,
}