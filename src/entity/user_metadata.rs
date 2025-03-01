use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserMetaData {
    pub providers: Option<Vec<String>>,
}
