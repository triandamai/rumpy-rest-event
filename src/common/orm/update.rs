use crate::common::mongo::get_db_name;
use crate::common::orm::orm::Orm;
use bson::oid::ObjectId;
use bson::{doc, DateTime, Document};
use chrono::NaiveDate;
use log::info;
use mongodb::{Client, ClientSession, Collection};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Update {
    orm: Orm,
    set: Option<Document>,
    inc: Option<Document>,
    dec: Option<Document>,
}

impl Update {
    pub fn from(from: &str) -> Self {
        Update {
            orm: Orm::new_default(from),
            set: None,
            inc: None,
            dec: None,
        }
    }

    pub fn set_str(mut self, column: &str, value: &str) -> Self {
        let mut set = self.set.unwrap_or(Document::new());
        set.insert(column, value);
        self.set = Some(set);
        self
    }

    pub fn set_datetime(mut self, column: &str, value: DateTime) -> Self {
        let mut set = self.set.unwrap_or(Document::new());
        set.insert(column, value);
        self.set = Some(set);
        self
    }

    pub fn set_naive_date(mut self, column: &str, value: &NaiveDate) -> Self {
        let mut set = self.set.unwrap_or(Document::new());
        set.insert(column, bson::to_bson(value).unwrap());
        self.set = Some(set);
        self
    }
    pub fn set_float(mut self, column: &str, value: &f64) -> Self {
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

    pub fn set_bool(mut self, column: &str, value: bool) -> Self {
        let mut set = self.set.unwrap_or(Document::new());
        set.insert(column, value);
        self.set = Some(set);
        self
    }
    pub fn set(mut self, doc: Document) -> Self {
        self.set = Some(doc);
        self
    }
    pub fn inc(mut self, doc: Document) -> Self {
        self.inc = Some(doc);
        self
    }
    pub fn dec(mut self, doc: Document) -> Self {
        self.dec = Some(doc);
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

    pub fn set_null(mut self, column: &str) -> Self {
        let mut set = self.set.unwrap_or(Document::new());
        set.insert(column, None::<i32>);
        self.set = Some(set);
        self
    }

    pub fn filter_doc(mut self, column: &str, value: Document) -> Self {
        let orm = self.orm.filter_doc(column, value);
        self.orm = orm;
        self
    }
    pub async fn execute_one(self, client: &Client) -> Result<u64, String> {
        let set = &self.set.clone().unwrap_or(Document::new());
        self.one(set, client).await
    }
    pub async fn execute_one_with_session(
        self,
        client: &Client,
        session: &mut ClientSession,
    ) -> Result<u64, String> {
        let set = &self.set.clone().unwrap_or(Document::new());
        self.one_with_session(set, client, session).await
    }
    pub async fn execute_many(self, client: &Client) -> Result<u64, String> {
        let set = &self.set.clone().unwrap_or(Document::new());
        self.many(set, client).await
    }

    pub async fn execute_many_with_session(
        self,
        client: &Client,
        session: &mut ClientSession,
    ) -> Result<u64, String> {
        let set = &self.set.clone().unwrap_or(Document::new());
        self.many_with_session(set, client, session).await
    }
    pub async fn one<T: Serialize>(self, update: T, client: &Client) -> Result<u64, String> {
        if self.orm.collection_name.is_empty() {
            return Err("Specify collection name before UPDATE...".to_string());
        }
        if self.orm.filter.len() < 1 && self.orm.filters_group.len() < 1 {
            return Err("Specify filter before UPDATE...".to_string());
        }

        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let query = self.orm.get_filter_as_doc();
        let mut doc = Document::new();

        if self.set.is_none() {
            let data = bson::to_document(&update);
            if data.is_err() {
                let err = data.unwrap_err().to_string();
                info!(target: "UPDATE 1","{:?}",err);
                return Err(err);
            }
            doc.insert("$set", data.unwrap());
        } else {
            doc.insert("$set", self.set.unwrap());
        }

        if self.inc.is_some() {
            doc.insert("$inc", self.inc.unwrap());
        }

        if self.dec.is_some() {
            doc.insert("$inc", self.dec.unwrap());
        }

        let save = collection.update_one(query, doc).await;

        if save.is_err() {
            let message = format!("{:?}", save.unwrap_err().kind);
            info!(target: "db::update::error","{}",message.clone());
            return Err(message);
        }

        let save = save.unwrap();

        // info!(target: "db::UPDATE::oke","Success update  data");
        Ok(save.modified_count)
    }

    pub async fn one_with_session<T: Serialize>(
        self,
        update: T,
        client: &Client,
        session: &mut ClientSession,
    ) -> Result<u64, String> {
        if self.orm.collection_name.is_empty() {
            return Err("Specify collection name before UPDATE...".to_string());
        }
        if self.orm.filter.len() < 1 && self.orm.filters_group.len() < 1 {
            return Err("Specify filter before UPDATE...".to_string());
        }
        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let query = self.orm.get_filter_as_doc();
        let mut doc = Document::new();

        if self.set.is_none() {
            let data = bson::to_document(&update);
            if data.is_err() {
                let err = data.unwrap_err().to_string();
                info!(target: "UPDATE 1","{:?}",err);
                return Err(err);
            }
            doc.insert("$set", data.unwrap());
        } else {
            doc.insert("$set", self.set.unwrap());
        }

        if self.inc.is_some() {
            doc.insert("$inc", self.inc.unwrap());
        }

        if self.dec.is_some() {
            doc.insert("$dec", self.dec.unwrap());
        }

        let save = collection.update_one(query, doc).session(session).await;

        if save.is_err() {
            let message = format!("{:?}", save.unwrap_err().kind);
            info!(target: "db::update::error","{}",message.clone());
            return Err(message);
        }

        let save = save.unwrap();

        // info!(target: "db::UPDATE::oke","Success update  data");
        Ok(save.modified_count)
    }

    pub async fn many<T: Serialize>(self, update: T, client: &Client) -> Result<u64, String> {
        //info!(target: "db::update","Start update  data");
        if self.orm.collection_name.is_empty() {
            info!(target:"db::UPDATE::error", "Specify collection name before UPDATE...");
            return Err("Specify collection name before UPDATE...".to_string());
        }
        if self.orm.filter.len() < 1 && self.orm.filters_group.len() < 1 {
            info!(target:"db::UPDATE::error", "Specify filter before UPDATE...");
            return Err("Specify filter before UPDATE...".to_string());
        }
        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let query = self.orm.get_filter_as_doc();
        let mut doc = Document::new();

        if self.set.is_none() {
            let data = bson::to_document(&update);
            if data.is_err() {
                let err = data.unwrap_err().to_string();
                info!(target: "UPDATE 1","{:?}",err);
                return Err(err);
            }
            doc.insert("$set", data.unwrap());
        } else {
            doc.insert("$set", self.set.unwrap());
        }

        if self.inc.is_some() {
            doc.insert("$inc", self.inc.unwrap());
        }

        if self.dec.is_some() {
            doc.insert("$dec", self.dec.unwrap());
        }

        let save = collection.update_many(query, doc).await;

        if save.is_err() {
            let message = format!("{:?}", save.unwrap_err().kind);
            info!(target: "db::update::error","{}",message.clone());
            return Err(message);
        }
        //info!(target: "db::get::ok","Success update  data");
        Ok(save.unwrap().modified_count)
    }

    pub async fn many_with_session<T: Serialize>(
        self,
        update: T,
        client: &Client,
        session: &mut ClientSession,
    ) -> Result<u64, String> {
        //info!(target: "db::update","Start update  data");
        if self.orm.collection_name.is_empty() {
            info!(target:"db::UPDATE::error", "Specify collection name before UPDATE...");
            return Err("Specify collection name before UPDATE...".to_string());
        }
        if self.orm.filter.len() < 1 && self.orm.filters_group.len() < 1 {
            info!(target:"db::UPDATE::error", "Specify filter before UPDATE...");
            return Err("Specify filter before UPDATE...".to_string());
        }
        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let query = self.orm.get_filter_as_doc();
        let mut doc = Document::new();

        if self.set.is_none() {
            let data = bson::to_document(&update);
            if data.is_err() {
                let err = data.unwrap_err().to_string();
                info!(target: "UPDATE 1","{:?}",err);
                return Err(err);
            }
            doc.insert("$set", data.unwrap());
        } else {
            doc.insert("$set", self.set.unwrap());
        }

        if self.inc.is_some() {
            doc.insert("$inc", self.inc.unwrap());
        }

        if self.dec.is_some() {
            doc.insert("$dec", self.dec.unwrap());
        }

        let save = collection.update_many(query, doc).session(session).await;

        if save.is_err() {
            let message = format!("{:?}", save.unwrap_err().kind);
            info!(target: "db::update::error","{}",message.clone());
            return Err(message);
        }
        //info!(target: "db::get::ok","Success update  data");
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
        let orm = self.orm.filter_bool(column, operator, value.clone());
        self.orm = orm;
        self
    }
    pub fn filter_null(mut self, column: &str, operator: Option<&str>) -> Self {
        let orm = self.orm.filter_null(column, operator);
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
        let orm = self.orm.filter_object_id(column, value);
        self.orm = orm;
        self
    }

    pub fn show_merging(self) -> Vec<Document> {
        self.orm.merge_field(true)
    }
}
