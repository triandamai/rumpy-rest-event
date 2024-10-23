use bson::{doc, from_document, Document};
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::entity::thread_attachment::{ThreadAttachment, ThreadAttachmentDTO};
use bson::serde_helpers::{hex_string_as_object_id, bson_datetime_as_rfc3339_string};
use log::info;
use mongodb::{Collection, Database};
use crate::common::api_response::PagingResponse;
use crate::common::bson::{serialize_object_id, deserialize_object_id};
use crate::entity::thread::{Thread, ThreadDTO, ThreadWithDetailDTO};
use crate::entity::user_credential::UserDTO;
use tokio_stream::StreamExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadComment {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "thread_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_id: Option<ObjectId>,
    #[serde(
        rename = "comment_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_id: Option<ObjectId>,
    #[serde(
        rename = "created_by_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by_id: Option<ObjectId>,
    pub reply_count: i64,
    pub up_vote_count: i64,
    pub down_vote_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    pub body: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCommentDTO {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "thread_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub thread_id: Option<ObjectId>,
    #[serde(
        default,
        rename = "comment_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub comment_id: Option<ObjectId>,
    #[serde(
        rename = "created_by_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub created_by_id: Option<ObjectId>,
    pub reply_count: i64,
    pub up_vote_count: i64,
    pub down_vote_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    pub body: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
    #[serde(
        default,
        rename = "comment_id",
        skip_serializing_if = "Option::is_none"
    )
    ]
    pub attachments: Option<Vec<ThreadAttachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCommentDetailDTO {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    #[serde(
        rename = "thread",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread: Option<ThreadDTO>,
    #[serde(
        rename = "reply_to",
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to: Option<Vec<ThreadCommentDTO>>,
    #[serde(
        rename = "created_by",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by: Option<UserDTO>,
    pub reply_count: i64,
    pub up_vote_count: i64,
    pub down_vote_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    pub body: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
    pub attachments: Vec<ThreadAttachmentDTO>,
}

impl ThreadComment {
    pub fn to_dto(&self) -> ThreadCommentDTO {
        ThreadCommentDTO {
            id: self.id.clone(),
            thread_id: self.thread_id.clone(),
            comment_id: self.comment_id.clone(),
            created_by_id: self.created_by_id.clone(),
            reply_count: self.reply_count.clone(),
            up_vote_count: self.up_vote_count.clone(),
            down_vote_count: self.down_vote_count.clone(),
            tags: self.tags.clone(),
            mentions: self.mentions.clone(),
            body: self.body.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            attachments: Some(vec![]),
        }
    }

    pub async fn find_one(query: Document, db: &Database) -> Option<ThreadComment> {
        let collection: Collection<ThreadComment> = db.collection("thread-comments");
        let find = collection
            .find_one(query)
            .await;
        find.map_or_else(|e| None, |value| value)
    }

    pub async fn save(&mut self, db: &Database) -> Option<ThreadComment> {
        let collection: Collection<ThreadComment> = db.collection("thread-comments");
        let save = collection
            .insert_one(&mut *self).await;
        if save.is_err() {
            return None;
        }
        let save = save.unwrap();
        self.id = save.inserted_id.as_object_id();
        Some(self.clone())
    }

    pub async fn delete(&mut self, query: Document, db: Database) -> Result<ThreadComment, String> {
        let collection: Collection<ThreadComment> = db.collection("thread-comments");
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
        let collection: Collection<ThreadComment> = db.collection("thread-comments");
        let save = collection
            .delete_one(query)
            .await;
        if save.is_err() {
            info!(target:"thread-attachments","error occurred while saving");
            return Err(save.unwrap_err().to_string());
        }
        Ok("Delete success".to_string())
    }

    pub async fn update_one(query: Document, data: Document, db: &Database) -> Result<String, String> {
        let collection: Collection<ThreadComment> = db.collection("thread-comments");
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

    pub async fn find_all_paging(query: Option<Document>, page: i64, size: i64, db: &Database) -> PagingResponse<ThreadCommentDetailDTO> {
        let collection: Collection<ThreadComment> = db.collection("thread-comments");
        let count_pipeline = match query {
            None => vec![doc! {"$count": "total"}],
            Some(_) => vec![
                query.clone().unwrap(),
                doc! {"$count": "total"}
            ]
        };

        let get_count = collection.aggregate(count_pipeline).await;


        let skip = (page - 1) * size;
        let count = match get_count {
            Err(e) => 0,
            Ok(mut value) =>
                match tokio_stream::StreamExt::try_next(&mut value).await {
                    Err(r) => 0,
                    Ok(csr) => match csr {
                        None => 0,
                        Some(c) => {
                            info!(target: "Thread::o","{:?}",c);
                            c.get_i32("total")
                                .unwrap_or(0)
                        }
                    }
                }
        };

        let pipeline = match query {
            None => vec![
                doc! {
                "$lookup": {
                    "from": "threads",
                    "localField": "thread_id",
                    "foreignField": "_id",
                    "as": "thread"
                }
            },
                doc! {
                "$lookup":{
                    "from":"thread-comments",
                    "localField":"comment_id",
                    "foreignField":"_id",
                    "as":"reply_to"
                }
            },
                doc! {
                "$lookup":{
                    "from":"user-iv",
                    "localField":"created_by_id",
                    "foreignField":"_id",
                    "as":"created_by"
                }
            },
                doc! {
                "$lookup":{
                    "from":"thread-attachments",
                    "localField":"_id",
                    "foreignField":"ref_id",
                    "as":"attachments"
                }
            },
                doc! {
                "$unwind":"$created_by"
            },
                doc! {
                "$unwind":"$thread"
            },
                doc! {
                "$sort": {
                    "created_at": -1 // Sort by created_at in descending order
                }
            },
                doc! {
                 "$skip":skip
            },
                doc! {
                "$limit":size,
            }
            ],
            Some(_) => vec![
                query.unwrap(),
                doc! {
                "$lookup": {
                    "from": "threads",
                    "localField": "thread_id",
                    "foreignField": "_id",
                    "as": "thread"
                }
            },
                doc! {
                "$lookup":{
                    "from":"thread-comments",
                    "localField":"comment_id",
                    "foreignField":"_id",
                    "as":"reply_to"
                }
            },
                doc! {
                "$lookup":{
                    "from":"user-iv",
                    "localField":"created_by_id",
                    "foreignField":"_id",
                    "as":"created_by"
                }
            },
                doc! {
                "$lookup":{
                    "from":"thread-attachments",
                    "localField":"_id",
                    "foreignField":"ref_id",
                    "as":"attachments"
                }
            },
                doc! {
                "$unwind":"$created_by"
            },
                doc! {
                "$unwind":"$thread"
            },
                doc! {
                "$sort": {
                    "created_at": -1 // Sort by created_at in descending order
                }
            },
                doc! {
                 "$skip":skip
            },
                doc! {
                "$limit":size,
            }
            ]
        };

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
        let mut result: Vec<ThreadCommentDetailDTO> = Vec::new();
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
            let tr = from_document::<ThreadCommentDetailDTO>(item.clone());

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
}