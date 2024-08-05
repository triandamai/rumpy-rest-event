use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateNewUserRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub auto_confirm: bool,
    pub username: String,
}

#[derive(Deserialize,Serialize)]
pub struct SearchByUsernameRequest{
    pub username:Option<String>
}
