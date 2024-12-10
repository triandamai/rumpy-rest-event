use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    pub full_name: String,
    pub email: String,
    pub gender: String,
    pub job_title: String,
}
