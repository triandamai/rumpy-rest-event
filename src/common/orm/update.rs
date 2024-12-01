use crate::common::orm::orm::Orm;
use bson::oid::ObjectId;
use bson::{doc, Document};
use log::info;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Update {
    orm: Orm,
    set: Option<Document>,
}

impl Update {
    pub fn from(from: &str) -> Self {
        Update {
            orm: Orm {
                collection_name: from.to_string(),
                filter: vec![],
                filters: Default::default(),
                current_filter: None,
                lookup: vec![],
                unwind: vec![],
                count: None,
                skip: None,
                limit: None,
            },
            set: None,
        }
    }

    pub fn set_str(mut self, column: &str, value: &str) -> Self {
        let mut set = self.set.unwrap_or(Document::new());
        set.insert(column, value);
        self.set = Some(set);
        self
    }
    pub fn set_number(mut self, column: &str, value: &i64) -> Self {
        let mut set = self.set.unwrap_or(Document::new());
        set.insert(column, value);
        self.set = Some(set);
        self
    }
    pub fn set_vec<T: Serialize>(mut self, column: &str, value: Vec<T>) -> Self {
        let mut set = self.set.unwrap_or(Document::new());
        set.insert(column, bson::to_document(&value).unwrap_or(Document::new()));
        self.set = Some(set);
        self
    }

    pub fn set_bool(mut self, column: &str, value: &bool) -> Self {
        let mut set = self.set.unwrap_or(Document::new());
        set.insert(column, value);
        self.set = Some(set);
        self
    }
    pub fn set_object_id(mut self, column: &str, value: &ObjectId) -> Self {
        let mut set = self.set.unwrap_or(Document::new());
        set.insert(column, value);
        self.set = Some(set);
        self
    }
    pub fn set_object<T: Serialize>(mut self, column: &str, value: T) -> Self {
        let mut set = self.set.unwrap_or(Document::new());
        set.insert(column, bson::to_document(&value).unwrap_or(Document::new()));
        self.set = Some(set);
        self
    }

    pub async fn execute_one(self, db: &Database) -> Result<u64, String> {
        let set = &self.set.clone().unwrap_or(Document::new());
        self.one(set, db).await
    }
    pub async fn execute_many(self, db: &Database) -> Result<u64, String> {
        let set = &self.set.clone().unwrap_or(Document::new());
        self.many(set, db).await
    }

    pub async fn one<T: Serialize>(self, update: T, db: &Database) -> Result<u64, String> {
        if self.orm.collection_name.is_empty() {
            return Err("Specify collection name before update...".to_string());
        }
        if self.orm.filter.len() < 1 && self.orm.filters.len() < 1 {
            return Err("Specify filter before update...".to_string());
        }
        let doc = bson::to_document(&update);
        if doc.is_err() {
            return Err("".to_string());
        }

        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let query = self.orm.get_filter_as_doc();

        info!(target: "sas","{}",query);
        let save = collection
            .update_one(
                query,
                doc! {
                    "$set":doc.unwrap()
                },
            )
            .await;

        if save.is_err() {
            return Err(save.unwrap_err().to_string());
        }

        let save = save.unwrap();

        info!(target: "db::update::oke","Success update data");
        Ok(save.modified_count)
    }

    pub async fn many<T: Serialize>(self, update: T, db: &Database) -> Result<u64, String> {
        info!(target: "db::update","Start update data");
        if self.orm.collection_name.is_empty() {
            info!(target:"db::update::error", "Specify collection name before update...");
            return Err("Specify collection name before update...".to_string());
        }
        if self.orm.filter.len() < 1 && self.orm.filters.len() < 1 {
            info!(target:"db::update::error", "Specify filter before update...");
            return Err("Specify filter before update...".to_string());
        }
        let doc = bson::to_document(&update);
        if doc.is_err() {
            let err_message = doc.unwrap_err().to_string();
            info!(target: "db::get::error","{}",err_message.clone());

            return Err(err_message);
        }

        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());

        let query = self.orm.get_filter_as_doc();

        let save = collection
            .update_many(query, doc! {"$set":doc.unwrap()})
            .await;

        if save.is_err() {
            let err_message = save.unwrap_err().to_string();
            info!(target: "db::get::error","{}",err_message.clone());

            return Err(err_message);
        }
        info!(target: "db::get::ok","Success update data");

        Ok(save.unwrap().modified_count)
    }

    //query

    pub fn or(mut self) -> Self {
        let orm = self.orm.or();
        self.orm = orm;
        self
    }

    pub fn and(mut self) -> Self {
        let orm = self.orm.and();
        self.orm = orm;
        self
    }

    pub fn filter_bool(mut self, column: &str, value: bool) -> Self {
        let orm = self.orm.filter_bool(column, None, value.clone());
        self.orm = orm;
        self
    }

    pub fn filter_array<T: Serialize>(mut self, column: &str, value: Vec<T>) -> Self {
        let orm = self.orm.filter_array(column, None, value);
        self.orm = orm;
        self
    }

    pub fn filter_number(mut self, column: &str, value: i64) -> Self {
        let orm = self.orm.filter_number(column, None, value);
        self.orm = orm;
        self
    }

    pub fn filter_string(mut self, column: &str, value: &str) -> Self {
        let orm = self.orm.filter_string(column, None, value);
        self.orm = orm;
        self
    }

    pub fn filter_object_id(mut self, column: &str, value: &ObjectId) -> Self {
        let orm = self.orm.filter_object_id(column, value);
        self.orm = orm;
        self
    }

    pub fn show_merging(self) -> Vec<Document> {
        self.orm.merge_field(true)
    }
}
