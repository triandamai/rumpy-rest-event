use crate::common::api_response::PagingResponse;
use crate::common::bson::{deserialize_object_id, serialize_object_id};
use bson::serde_helpers::{bson_datetime_as_rfc3339_string, hex_string_as_object_id};
use chrono::NaiveDate;
use futures::stream::TryStreamExt;
use log::info;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{DateTime, Document};
use mongodb::error::Error;
use mongodb::options::FindOptions;
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
    WaitingConfirmation,
    Suspended,
    Locked,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthProvider {
    Basic,
    Google,
    Facebook,
    Apple,
    Twitter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredential {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub status: UserStatus,
    pub date_of_birth: Option<NaiveDate>,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
    pub username: String,
    pub deleted: bool,
    pub auth_provider: AuthProvider,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub full_name: String,
    pub email: String,
    pub status: UserStatus,
    pub date_of_birth: Option<NaiveDate>,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
    pub username: String,
    pub deleted: bool,
    pub auth_provider: AuthProvider,
}

impl UserCredential {
    pub fn to_dto(&self) -> UserDTO {
        UserDTO {
            id: self.id.clone(),
            full_name: self.full_name.clone(),
            email: self.email.clone(),
            status: self.status.clone(),
            date_of_birth: self.date_of_birth.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            username: self.username.clone(),
            deleted: self.deleted.clone(),
            auth_provider: self.auth_provider.clone(),
        }
    }
    pub fn redact_password(&mut self) -> UserCredential {
        self.password = "****".to_string();
        self.clone()
    }
    pub fn is_waiting_confirmation(&self) -> bool {
        match self.status {
            UserStatus::Active => false,
            UserStatus::Inactive => false,
            UserStatus::WaitingConfirmation => true,
            UserStatus::Suspended => false,
            UserStatus::Locked => false
        }
    }
    pub fn is_active(&self) -> bool {
        match self.status {
            UserStatus::Active => true,
            UserStatus::Inactive => false,
            UserStatus::WaitingConfirmation => false,
            UserStatus::Suspended => false,
            UserStatus::Locked => false
        }
    }

    pub fn get_status_message(&self) -> &str {
        match self.status {
            UserStatus::Active => "",
            UserStatus::Inactive => "Akun kamu Sudah tidak aktif",
            UserStatus::Locked => "Akun kamu Dikunci",
            UserStatus::Suspended => "Akun kamu Disuspen",
            UserStatus::WaitingConfirmation => "Akun kamu Sedang Menunggu Konfirmasi",
        }
    }

    pub async fn email_exist(email: &String, db: &Database) -> bool {
        let collection: Collection<UserCredential> = db.collection("user-iv");
        let user = collection.find_one(
            doc! {
                "email":doc!{
                    "$eq":email.clone()
                }
            }
        ).await;
        if user.is_err() {
            info!(target:"[UserCredential::email_exist] -> ","{}",user.unwrap_err());
            return false;
        }
        let user = user.unwrap();
        if user.is_some() {
            return false;
        }
        true
    }

    pub async fn update_one(query: Document, data: Document, db: &Database) -> Result<String, String> {
        let collection: Collection<UserCredential> = db.collection("user-iv");
        let user = collection
            .update_one(query, data)
            .await;
        if user.is_err() {
            info!(target:"[UserCredential::find_one] -> ","{}",user.unwrap_err().to_string());
            return Err("Gagal merubah user".to_string());
        }
        let user = user.unwrap();
        Ok("Berhasil merubah user".to_string())
    }

    pub async fn find_one(doc: Document, db: &Database) -> Option<UserCredential> {
        let collection: Collection<UserCredential> = db.collection("user-iv");
        let user = collection
            .find_one(doc)
            .await;
        if user.is_err() {
            info!(target:"[UserCredential::find_one] -> ","{}",user.unwrap_err().to_string());
            return None;
        }
        let user = user.unwrap();
        user
    }

    pub async fn find_all(doc: Document, db: &Database) -> Vec<UserCredential> {
        let collection: Collection<UserCredential> = db.collection("user-iv");
        let users = collection.find(doc)
            .await;

        if users.is_err() {
            info!(target:"[UserCredential::find_all] -> ","{}",users.unwrap_err());
            return Vec::new();
        }
        let users = users.unwrap().try_collect().await;
        if users.is_err() {
            info!(target: "[UserCredential::find_all] -> ","{}",users.unwrap_err());
            return Vec::new();
        }
        info!(target:"[UserCredential::find_all] -> ","success");
        users.unwrap()
    }

    pub async fn find_with_paging(document: Document, page: i64, size: i64, db: &Database) -> PagingResponse<UserDTO> {
        let collection: Collection<UserCredential> = db.collection("user-iv");
        let get_count = collection.count_documents(document.clone()).await;
        let skip = (page - 1) * size;
        if get_count.is_err() {
            return PagingResponse {
                total_pages: 0,
                total_items: 0,
                items: Vec::new(),
                size: 0,
                page: 0,
            };
        }
        let get_count = get_count.unwrap();
        let find_options = FindOptions::builder()
            .skip(skip as u64)
            .limit(size)
            .build();


        let find_data = collection
            .find(document)
            .with_options(find_options)
            .await;
        if find_data.is_err() {
            return PagingResponse {
                total_pages: 0,
                total_items: 0,
                items: Vec::new(),
                size: 0,
                page: 0,
            };
        }
        let find_data = find_data.unwrap();
        let data = find_data.try_collect().await;
        if data.is_err() {
            return PagingResponse {
                total_pages: 0,
                total_items: 0,
                items: Vec::new(),
                size: 0,
                page: 0,
            };
        }
        let data: Vec<UserCredential> = data.unwrap();
        let transform = data.iter().map(|value| value.clone().to_dto()).collect();
        let count_pages = (get_count.clone() as f64 / size as f64).ceil() as u32;
        PagingResponse {
            total_pages: count_pages as i64,
            total_items: get_count as i64,
            items: transform,
            size,
            page,
        }
    }

    pub async fn save(&mut self, db: &Database) -> Result<UserDTO, String> {
        info!(target:"[UserCredential::save] ->","{} into db",self.email.clone());
        let collection: Collection<UserCredential> = db.collection("user-iv");

        let saved = collection
            .insert_one(&mut *self)
            .await;

        if saved.is_err() {
            let message = saved.unwrap_err().to_string();
            info!(target: "[UserCredential::save] ->","{}",message);
            return Err("Gagal menyimpan user ke db".to_string());
        }
        info!(target: "[UserCredential::save] ->","success");
        self.id = Some(saved.unwrap().inserted_id.as_object_id().unwrap());
        Ok(self.clone().to_dto())
    }
}



