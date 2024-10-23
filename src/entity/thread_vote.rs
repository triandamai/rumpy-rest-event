use crate::common::bson::{deserialize_object_id, serialize_object_id};
use bson::oid::ObjectId;
use bson::{doc, DateTime, Document};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use bson::serde_helpers::bson_datetime_as_rfc3339_string;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteType {
    Up,
    Down,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadVote {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub ref_id: Option<ObjectId>,
    pub vote_by: Option<ObjectId>,
    pub vote_type: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadVoteDTO {
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
    #[serde(
        rename = "vote_by",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub vote_by: Option<ObjectId>,
    pub vote_type: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}
impl ThreadVote {
    pub fn to_dto(&self) -> ThreadVoteDTO {
        ThreadVoteDTO {
            id: self.id.clone(),
            ref_id: self.ref_id.clone(),
            vote_by: self.vote_by.clone(),
            vote_type: self.vote_type.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }

    pub async fn save(&mut self, db: &Database) -> Option<ThreadVote> {
        let collection: Collection<ThreadVote> = db.collection("thread-votes");


        let find = collection
            .find_one(doc! {
                "ref_id": self.ref_id.clone(),
                "vote_by": self.vote_by.clone(),
                "vote_type": self.vote_type.clone(),
            })
            .await
            .unwrap_or(None);

        if find.is_none() {
            let save = collection
                .insert_one(&mut *self)
                .await;
            if save.is_err() {
                return None;
            }
            let save = save.unwrap();
            self.id = save.inserted_id.as_object_id();
            return Some(self.clone());
        }
        let find = find.unwrap();
        Some(find)
    }
    pub async fn delete(&mut self, db: &Database) -> Result<ThreadVote, String> {
        let collection: Collection<ThreadVote> = db.collection("thread-votes");
        let save = collection
            .delete_one(doc! {
                "_id": &self.id
            })
            .await;
        if save.is_err() {
            return Err("Failed to delete".to_string());
        }
        Ok(self.clone())
    }
    pub async fn delete_one(doc: Document, db: &Database) -> Result<String, String> {
        let collection: Collection<ThreadVote> = db.collection("thread-votes");
        let save = collection
            .delete_one(doc)
            .await;
        if save.is_err() {
            return Err("Failed to delete".to_string());
        }
        Ok("Success".to_string())
    }
}