use bson::DateTime;
use bson::oid::ObjectId;
use log::info;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Configuration {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub namespace: String,
    pub name: String,
    pub group: String,
    pub content: Option<serde_json::Value>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Configuration {
    pub fn extract_content<T: DeserializeOwned>(self) -> Option<T> {
        match self.content {
            Some(value) => {
                let dto = serde_json::from_value::<T>(value);
                match dto {
                    Ok(result) => Some(result),
                    Err(e) => {
                        info!(target:"extract_content_config","failed to deserialize content {:?}",e);
                        None
                    },
                }
            }
            None => None,
        }
    }
}
