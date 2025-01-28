use crate::common::orm::orm::Orm;
use bson::oid::ObjectId;
use bson::{doc, Document};
use mongodb::{ClientSession, Collection, Database};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Delete {
    orm: Orm,
}

impl Delete {
    pub fn from(from: &str) -> Self {
        Delete {
            orm: Orm::new_default(from),
        }
    }

    pub async fn one(self, db: &Database) -> Result<u64, String> {
        if self.orm.collection_name.is_empty() {
            return Err("Specify collection name before deleting...".to_string());
        }
        if self.orm.filter.len() < 1 && self.orm.filters_group.len() < 1 {
            return Err("Specify filter before deleting...".to_string());
        }
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let query = self.orm.get_filter_as_doc();

        let save = collection.delete_one(query).await;

        if save.is_err() {
            return Err(save.unwrap_err().to_string());
        }

        Ok(save.unwrap().deleted_count)
    }

    pub async fn one_with_session(
        self,
        db: &Database,
        session: &mut ClientSession,
    ) -> Result<u64, String> {
        if self.orm.collection_name.is_empty() {
            return Err("Specify collection name before deleting...".to_string());
        }
        if self.orm.filter.len() < 1 && self.orm.filters_group.len() < 1 {
            return Err("Specify filter before deleting...".to_string());
        }
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let query = self.orm.get_filter_as_doc();

        let save = collection.delete_one(query).session(session).await;

        if save.is_err() {
            return Err(save.unwrap_err().to_string());
        }

        Ok(save.unwrap().deleted_count)
    }

    pub async fn many(self, db: &Database) -> Result<u64, String> {
        if self.orm.filter.len() < 1 && self.orm.filters_group.len() < 1 {
            return Err("Specify filter before deleting...".to_string());
        }

        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let query = self.orm.get_filter_as_doc();

        let save = collection.delete_many(query).await;

        if save.is_err() {
            return Err(save.unwrap_err().to_string());
        }

        Ok(save.unwrap().deleted_count)
    }

    pub async fn many_with_session(
        self,
        db: &Database,
        session: &mut ClientSession,
    ) -> Result<u64, String> {
        if self.orm.filter.len() < 1 && self.orm.filters_group.len() < 1 {
            return Err("Specify filter before deleting...".to_string());
        }

        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let query = self.orm.get_filter_as_doc();

        let save = collection.delete_many(query).session(session).await;

        if save.is_err() {
            return Err(save.unwrap_err().to_string());
        }

        Ok(save.unwrap().deleted_count)
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
