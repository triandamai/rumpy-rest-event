use crate::common::api_response::PagingResponse;
use crate::common::bson::{deserialize_object_id, serialize_object_id};
use crate::entity::thread_attachment::{ThreadAttachment, ThreadAttachmentDTO};
use crate::entity::user_credential::{UserCredential, UserDTO};
use bson::serde_helpers::{bson_datetime_as_rfc3339_string, hex_string_as_object_id};
use log::info;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{bson, doc, from_document, DateTime, Document};
use mongodb::error::Error;
use mongodb::{Collection, Cursor, Database};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "created_by_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by_id: Option<ObjectId>,
    #[serde(
        rename = "quote_thread_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_thread_id: Option<ObjectId>,
    pub title: String,
    pub content: String,
    pub watch_count: i64,
    pub comment_count: i64,
    pub up_vote_count: i64,
    pub down_vote_count: i64,
    #[serde(
        default,
        rename = "mentions",
        skip_serializing_if = "Option::is_none"
    )]
    pub mentions: Option<Vec<String>>,
    #[serde(
        default,
        rename = "tags",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Vec<String>>,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadDTO {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "created_by_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub created_by_id: Option<ObjectId>,
    #[serde(
        rename = "quote_thread_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub quote_thread_id: Option<ObjectId>,
    pub title: String,
    pub content: String,
    pub watch_count: i64,
    pub comment_count: i64,
    pub up_vote_count: i64,
    pub down_vote_count: i64,
    #[serde(
        default,
        rename = "mentions",
        skip_serializing_if = "Option::is_none"
    )]
    pub mentions: Option<Vec<String>>,
    #[serde(
        default,
        rename = "tags",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Vec<String>>,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadWithDetailDTO {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub created_by: Option<UserDTO>,
    pub quote_thread: Option<Vec<ThreadDTO>>,
    pub attachments: Option<Vec<ThreadAttachmentDTO>>,
    pub title: String,
    pub content: String,
    pub watch_count: i64,
    pub comment_count: i64,
    pub up_vote_count: i64,
    pub down_vote_count: i64,
    #[serde(
        default,
        rename = "mentions",
        skip_serializing_if = "Option::is_none"
    )]
    pub mentions: Option<Vec<String>>,
    #[serde(
        default,
        rename = "tags",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Vec<String>>,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}

impl ThreadWithDetailDTO {
    pub fn to_basic(&self) -> Thread {
        Thread {
            id: self.id,
            title: self.title.clone(),
            content: self.content.clone(),
            watch_count: self.watch_count.clone(),
            comment_count: self.comment_count.clone(),
            up_vote_count: self.up_vote_count.clone(),
            down_vote_count: self.down_vote_count.clone(),
            mentions: self.mentions.clone(),
            tags: self.tags.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_id: match self.created_by.clone() {
                None => None,
                Some(value) => value.id
            },
            quote_thread_id: if self.quote_thread.clone().unwrap_or(vec![]).len() > 0 {
                let q = self.quote_thread.clone().unwrap();;
                if q.get(0).is_some() {
                    if q.get(0).unwrap().id.is_some() {
                        Some(q.get(0).unwrap().id.unwrap())
                    } else { None }
                } else { None }
            } else { None },
        }
    }
}

impl Thread {
    pub fn create_field_lookup_thread() -> Document {
        doc! {"$lookup": {
                    "from": "threads",
                    "localField": "quote_thread_id",
                    "foreignField": "_id",
                    "as": "quote_thread"
                }
        }
    }
    pub fn create_field_lookup_user() -> Document {
        doc! {"$lookup":{
            "from":"user-iv",
            "localField":"created_by_id",
            "foreignField":"_id",
            "as":"created_by"
        }}
    }


    pub fn create_field_lookup_attachment() -> Document {
        doc! {
                "$lookup":{
                    "from":"thread-attachments",
                    "localField":"_id",
                    "foreignField":"ref_id",
                    "as":"attachments"
                }
            }
    }

    pub fn create_field_unwind(column: String) -> Document {
        doc! {"$unwind":column}
    }
    pub fn create_field_limit(limit: i64) -> Document {
        doc! {"$limit":limit}
    }
    pub fn create_field_skip(skip: i64) -> Document {
        doc! {"$skip":skip}
    }

    pub fn create_field_sort_desc() -> Document {
        doc! {"$sort": {
            "created_at": -1 // Sort by created_at in descending order
        }}
    }
    pub fn to_dto(&self) -> ThreadDTO {
        ThreadDTO {
            id: self.id.clone(),
            created_by_id: self.created_by_id.clone(),
            quote_thread_id: self.quote_thread_id.clone(),
            title: self.title.clone(),
            content: self.content.clone(),
            watch_count: self.watch_count.clone(),
            comment_count: self.comment_count.clone(),
            up_vote_count: self.up_vote_count.clone(),
            down_vote_count: self.down_vote_count.clone(),
            mentions: self.mentions.clone(),
            tags: self.tags.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
    pub async fn find_one_basic(query: Vec<Document>, db: &Database)->Option<ThreadDTO>{
        let collection: Collection<Thread> = db.collection("threads");
        let find = collection
            .aggregate(query).await;
        if find.is_err() {
            info!(target: "Thread::find_one","{}",find.unwrap_err().to_string());
            return None;
        }
        let mut csr = find.unwrap();
        let thread = csr.next().await;
        if thread.is_none() {
            return None;
        }
        let thread = thread.unwrap();
        if thread.is_err() {
            return None;
        }
        let thread = thread.unwrap();
        info!(target:"find_one","{:?}",thread.clone());
        let transform = from_document::<ThreadDTO>(thread);
        info!(target:"find_one","{:?}",transform.clone());
        match transform {
            Ok(value) => Some(value),
            Err(_) => None
        }
    }
    pub async fn find_one(query: Vec<Document>, db: &Database) -> Option<ThreadWithDetailDTO> {
        let collection: Collection<Thread> = db.collection("threads");
        let find = collection
            .aggregate(query).await;
        if find.is_err() {
            info!(target: "Thread::find_one","{}",find.unwrap_err().to_string());
            return None;
        }
        let mut csr = find.unwrap();
        let thread = csr.next().await;
        if thread.is_none() {
            return None;
        }
        let thread = thread.unwrap();
        if thread.is_err() {
            return None;
        }
        let thread = thread.unwrap();
        info!(target:"find_one","{:?}",thread.clone());
        let transform = from_document::<ThreadWithDetailDTO>(thread);
        info!(target:"find_one","{:?}",transform.clone());
        match transform {
            Ok(value) => Some(value),
            Err(_) => None
        }
    }

    pub async fn find(pipeline: Vec<Document>, mut query_count: Vec<Document>, page: i64, size: i64, db: &Database) -> PagingResponse<ThreadWithDetailDTO> {
        let collection: Collection<Thread> = db.collection("threads");

        query_count.push(doc! {"$count": "total"});

        let get_count = collection.aggregate(query_count).await;

        let count = match get_count {
            Ok(mut value) => StreamExt::try_next(&mut value)
                .await
                .map_or_else(|_| None, |document| document
                    .map_or_else(|| None, |doc| Some(
                        doc
                            .get_i32("total")
                            .unwrap_or(0)
                    )),
                ),
            Err(_) => None
        }.unwrap_or(0);

        let find = collection
            .aggregate(pipeline).await;

        if find.is_err() {
            info!(target: "Thread::find err","{}",find.unwrap_err().to_string());
            return PagingResponse {
                total_items: count as i64,
                total_pages: 0,
                page,
                size,
                items: vec![],
            };
        }
        let mut result = Vec::new();
        let mut csr = find.unwrap();

        let extract = csr.collect::<Result<Vec<Document>, _>>().await;

        if extract.is_err() {
            info!(target: "Thread::find err","{}",extract.unwrap_err().to_string());
            return PagingResponse {
                total_items: count as i64,
                total_pages: 0,
                page,
                size,
                items: vec![],
            };
        }
        let extract = extract.unwrap();

        for item in extract {
            let tr = from_document::<ThreadWithDetailDTO>(item.clone());

            if tr.is_ok() {
                result.push(tr.unwrap());
            } else {
                info!(target:"extract from doc:","{:?}",tr.unwrap_err());
            }
        }

        let count_pages = (count.clone() as f64 / size as f64).ceil() as u32;

        PagingResponse {
            total_items: count as i64,
            total_pages: count_pages as i64,
            page,
            size,
            items: result,
        }
    }

    pub async fn save(&mut self, db: &Database) -> Option<Thread> {
        let collection: Collection<Thread> = db.collection("threads");
        let saved = collection.insert_one(&mut *self)
            .await;

        if saved.is_err() {
            info!(target: "Thread::save","{}",saved.unwrap_err().to_string());
            return None;
        }

        let saved_id = saved.unwrap();
        self.id = Some(saved_id.inserted_id.as_object_id().unwrap());

        Some(self.clone())
    }

    pub async fn update_one(query: Document, data: Document, db: &Database) -> Result<String, String> {
        let collection: Collection<UserCredential> = db.collection("threads");
        let user = collection
            .update_one(query, data)
            .await;
        if user.is_err() {
            info!(target:"[UserCredential::find_one] -> ","{}",user.unwrap_err().to_string());
            return Err("Gagal merubah user".to_string());
        }
        let user = user.unwrap();
        Ok("Berhasil merubah thread".to_string())
    }
}