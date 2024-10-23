use crate::common::bson::{deserialize_object_id, serialize_object_id};
use bson::serde_helpers::{bson_datetime_as_rfc3339_string, hex_string_as_object_id};
use futures::stream::TryStreamExt;
use log::info;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{DateTime, Document};
use mongodb::error::Error;
use mongodb::{Collection, Cursor, Database};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadAttachment {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(rename = "ref_id", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<ObjectId>,
    pub filename: String,
    pub mime_type: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadAttachmentDTO {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "ref_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub ref_id: Option<ObjectId>,
    pub filename: String,
    pub mime_type: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}

impl ThreadAttachmentDTO {
    pub fn to_basic(&self) -> ThreadAttachment {
        ThreadAttachment {
            id: self.id,
            ref_id: self.ref_id.clone(),
            filename: self.filename.clone(),
            mime_type: self.mime_type.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl ThreadAttachment {
    pub fn to_dto(&self) -> ThreadAttachmentDTO {
        ThreadAttachmentDTO {
            id: self.id.clone(),
            ref_id: self.ref_id.clone(),
            filename: self.filename.clone(),
            mime_type: self.mime_type.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
    pub async fn find_one(doc: Document, db: &Database) -> Option<ThreadAttachment> {
        let collection: Collection<ThreadAttachment> = db.collection("thread-attachments");
        let find = collection.find_one(doc).await;
        if find.is_err() {
            return None;
        }
        find.unwrap()
    }
    pub async fn find_all(doc: Document, db: &Database) -> Vec<ThreadAttachment> {
        let collection: Collection<ThreadAttachment> = db.collection("thread-attachments");
        let find: Result<Cursor<ThreadAttachment>, Error> = collection.find(doc).await;
        if find.is_err() {
            return Vec::new();
        }
        let mut find = find.unwrap();
        let mut result = Vec::new();
        while let Ok(next) = tokio_stream::StreamExt::try_next(&mut find).await {
            if let Some(value) = next {
                result.push(value);
            }
        }
        result
    }

    pub async fn save(&mut self, db: &Database) -> Option<ThreadAttachment> {
        let collection: Collection<ThreadAttachment> = db.collection("thread-attachments");
        let save = collection
            .insert_one(&mut *self)
            .await;
        if save.is_err() {
            info!(target:"thread-attachments","error occurred while saving");
            return None;
        }
        self.id = save.unwrap().inserted_id.as_object_id();
        Some(self.clone())
    }

    pub async fn update_one(query: Document, data: Document, db: &Database) -> Result<String, String> {
        let collection: Collection<ThreadAttachment> = db.collection("thread-attachments");
        let save = collection
            .update_one(query, data)
            .await;
        if save.is_err() {
            info!(target:"thread-attachments","error occurred while saving");
            return Err("Failed to update".to_string());
        }

        Ok("Berhasil update".to_string())
    }

    pub async fn delete(&mut self, query: Document, db: &Database) -> Result<ThreadAttachment, String> {
        let collection: Collection<ThreadAttachment> = db.collection("thread-attachments");
        let save = collection
            .delete_one(query)
            .await;
        if save.is_err() {
            info!(target:"thread-attachments","error occurred while saving");
            return Err(save.unwrap_err().to_string());
        }
        Ok(self.clone())
    }
    pub async fn delete_one(query: Document, db: &Database) -> Result<String, String> {
        let collection: Collection<ThreadAttachment> = db.collection("thread-attachments");
        let save = collection
            .delete_one(query)
            .await;
        if save.is_err() {
            info!(target:"thread-attachments","error occurred while saving");
            return Err(save.unwrap_err().to_string());
        }
        Ok("Delete success".to_string())
    }
}