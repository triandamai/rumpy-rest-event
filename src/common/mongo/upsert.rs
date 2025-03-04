use crate::common::mongo::filter::FilterGroup;
use crate::common::mongo::DB;
use crate::common::orm::get_db_name;
use crate::common::orm::orm::Orm;
use bson::{Bson, Document};
use log::info;
use mongodb::{Client, ClientSession, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Upsert {
    orm: DB,
    set: Option<Document>,
    inc: Option<Document>,
}

impl Upsert {
    pub fn from(from: &str) -> Self {
        Upsert {
            orm: DB::get(from),
            set: None,
            inc: None,
        }
    }

    pub fn set(mut self, doc: Document) -> Self {
        self.set = Some(doc);
        self
    }

    pub fn set_value<T: Into<Bson>>(mut self, column: &str, value: T) -> Self {
        let mut doc = self.set.clone().unwrap_or(Document::new());
        doc.insert(column, value.into());
        self.set = Some(doc);
        self
    }

    pub fn inc(mut self, doc: Document) -> Self {
        self.inc = Some(doc);
        self
    }

    pub fn inc_value<T: Into<Bson>>(mut self, column: &str, value: T) -> Self {
        let mut doc = self.inc.clone().unwrap_or(Document::new());
        doc.insert(column, value.into());
        self.inc = Some(doc);
        self
    }

    pub fn filter<T: Into<FilterGroup>>(mut self, filters: Vec<T>) -> Self {
        self.orm = self.orm.filter(filters);
        self
    }

    pub async fn execute(self, client: &Client) -> Result<u64, String> {
        let set = &self.set.clone().unwrap_or(Document::new());
        self.one(set, client).await
    }
    pub async fn execute_with_session(
        self,
        client: &Client,
        session: &mut ClientSession,
    ) -> Result<u64, String> {
        let set = &self.set.clone().unwrap_or(Document::new());
        self.one_with_session(set, client, session).await
    }

    pub async fn one<T: Serialize>(self, update: T, client: &Client) -> Result<u64, String> {
        if self.orm.collection.is_empty() {
            return Err("Specify collection name before UPDATE...".to_string());
        }
        if let None = self.orm.filter {
            return Err("Specify filter before UPDATE...".to_string());
        }

        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(self.orm.collection.as_str());
        let query = self.orm.populate_filter();
        let mut doc = Document::new();

        if let None = self.set {
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

        if let Some(inc) = self.inc {
            doc.insert("$inc", inc);
        }

        let save = collection.replace_one(query, doc).await;

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
        if self.orm.collection.is_empty() {
            return Err("Specify collection name before UPDATE...".to_string());
        }
        if let None = self.orm.filter {
            return Err("Specify filter before UPDATE...".to_string());
        }
        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(self.orm.collection.as_str());
        let query = self.orm.populate_filter();
        let mut doc = Document::new();

        if let None = self.set {
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

        if let Some(inc) = self.inc {
            doc.insert("$inc", inc);
        }

        let save = collection.replace_one(query, doc).session(session).await;

        if save.is_err() {
            let message = format!("{:?}", save.unwrap_err().kind);
            info!(target: "db::update::error","{}",message.clone());
            return Err(message);
        }

        let save = save.unwrap();

        // info!(target: "db::UPDATE::oke","Success update  data");
        Ok(save.modified_count)
    }
    pub fn show_merging(self) -> (Vec<Document>, Vec<Document>) {
        self.orm.populate_pipeline()
    }
}
