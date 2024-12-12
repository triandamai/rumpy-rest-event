use crate::common::orm::orm::Orm;
use crate::common::orm::DB_NAME;
use bson::oid::ObjectId;
use bson::{doc, Document};
use log::info;
use mongodb::{Client, ClientSession, Collection};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Replace {
    orm: Orm,
}

impl Replace {
    pub fn from(from: &str) -> Self {
        Replace {
            orm: Orm {
                collection_name: from.to_string(),
                filter: vec![],
                filters: Default::default(),
                current_filter: None,
                lookup: vec![],
                unwind: vec![],
                sort: None,
                count: None,
                skip: None,
                limit: None,
            },
        }
    }
    pub async fn one<T: Serialize>(self, update: T, client: &Client) -> Result<u64, String> {
        //info!(target: "db::replace","Starting replace data");
        if self.orm.collection_name.is_empty() {
            info!(target: "db::replace::error","Replace collection name is empty");
            return Err("Specify collection name before replace...".to_string());
        }
        if self.orm.filter.len() < 1 && self.orm.filters.len() < 1 {
            info!(target: "db::replace::error","Replace filter is empty");
            return Err("Specify filter before replace...".to_string());
        }
        let doc = bson::to_document(&update);
        if doc.is_err() {
            let err_message = doc.unwrap_err().to_string();
            info!(target: "db::replace::error","{}",err_message.clone());
            return Err(err_message);
        }
        let db = client.database(DB_NAME);
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let query = self.orm.get_filter_as_doc();
        let save = collection.replace_one(query, doc.unwrap()).await;

        if save.is_err() {
            let err_message = save.unwrap_err().to_string();
            info!(target: "db::replace::error","{}",err_message.clone());
            return Err(err_message);
        }

        //info!(target: "db::replace::ok","Finished replace data");
        Ok(save.unwrap().modified_count)
    }

    pub async fn one_with_session<T: Serialize>(
        self,
        update: T,
        client: &Client,
        session: &mut ClientSession,
    ) -> Result<u64, String> {
        //info!(target: "db::replace","Starting replace data");
        if self.orm.collection_name.is_empty() {
            info!(target: "db::replace::error","Replace collection name is empty");
            return Err("Specify collection name before replace...".to_string());
        }
        if self.orm.filter.len() < 1 && self.orm.filters.len() < 1 {
            info!(target: "db::replace::error","Replace filter is empty");
            return Err("Specify filter before replace...".to_string());
        }
        let doc = bson::to_document(&update);
        if doc.is_err() {
            let err_message = doc.unwrap_err().to_string();
            info!(target: "db::replace::error","{}",err_message.clone());
            return Err(err_message);
        }
        let db = client.database(DB_NAME);
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let query = self.orm.get_filter_as_doc();
        let save = collection
            .replace_one(query, doc.unwrap())
            .session(session)
            .await;

        if save.is_err() {
            let err_message = save.unwrap_err().to_string();
            info!(target: "db::replace::error","{}",err_message.clone());
            return Err(err_message);
        }

        //info!(target: "db::replace::ok","Finished replace data");
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

    pub fn filter_bool(mut self, column: &str, operator: Option<&str>, value: bool) -> Self {
        let orm = self.orm.filter_bool(column, operator, value);
        self.orm = orm;
        self
    }

    pub fn filter_array<T: Serialize>(
        mut self,
        column: &str,
        operator: Option<&str>,
        value: Vec<T>,
    ) -> Self {
        let orm = self.orm.filter_array(column, operator, value);
        self.orm = orm;
        self
    }

    pub fn filter_number(mut self, column: &str, operator: Option<&str>, value: i64) -> Self {
        let orm = self.orm.filter_number(column, operator, value);
        self.orm = orm;
        self
    }

    pub fn filter_string(mut self, column: &str, operator: Option<&str>, value: &str) -> Self {
        let orm = self.orm.filter_string(column, operator, value);
        self.orm = orm;
        self
    }

    pub fn filter_object_id(mut self, column: &str, value: &ObjectId) -> Self {
        let orm = self.orm.filter_object_id_with_equal(column, value);
        self.orm = orm;
        self
    }

    pub fn show_merging(self) -> Vec<Document> {
        self.orm.merge_field(true)
    }
}
