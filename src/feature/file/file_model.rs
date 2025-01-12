use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryGetFile {
    pub file_name: Option<String>,
}
