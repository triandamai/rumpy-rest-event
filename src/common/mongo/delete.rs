use crate::common::mongo::filter::FilterGroup;
use crate::common::mongo::DB;
use crate::common::orm::get_db_name;
use crate::common::orm::orm::Orm;
use bson::{Bson, Document};
use log::info;
use mongodb::{Client, ClientSession, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Delete {
    orm: DB
}

impl Delete {
    pub fn from(from: &str) -> Self {
        Delete {
            orm: DB::get(from),
        }
    }

    pub fn filter<T: Into<FilterGroup>>(mut self, filters: Vec<T>) -> Self {
        self.orm = self.orm.filter(filters);
        self
    }

    pub async fn one(self, client: &Client) -> Result<u64, String> {
        if self.orm.collection.is_empty() {
            return Err("Specify collection name before deleting...".to_string());
        }
        if let None = self.orm.filter {
            return Err("Specify filter before deleting...".to_string());
        }
        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(&self.orm.collection);
        let query = self.orm.populate_filter();

        let save = collection.delete_one(query).await;

        if let Err(why) = save {
            let message = format!("{:?}", why.kind);
            info!(target: "db::delete::error","{}",message.clone());
            return Err(message);
        }

        Ok(save.unwrap().deleted_count)
    }

    pub async fn one_with_session(
        self,
        client: &Client,
        session: &mut ClientSession,
    ) -> Result<u64, String> {
        if self.orm.collection.is_empty() {
            return Err("Specify collection name before deleting...".to_string());
        }
        if let None = self.orm.filter {
            return Err("Specify filter before deleting...".to_string());
        }
        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(&self.orm.collection);
        let query = self.orm.populate_filter();

        let save = collection.delete_one(query).session(session).await;

        if let Err(why) = save{
            let message = format!("{:?}", why.kind);
            info!(target: "db::delete::error","{}",message.clone());
            return Err(message);
        }

        Ok(save.unwrap().deleted_count)
    }

    pub async fn many(self, client: &Client) -> Result<u64, String> {

        if let None = self.orm.filter {
            return Err("Specify filter before deleting...".to_string());
        }

        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(&self.orm.collection);
        let query = self.orm.populate_filter();

        let save = collection.delete_many(query).await;

        if let Err(why) = save {
            let message = format!("{:?}", why.kind);
            info!(target: "db::delete::error","{}",message.clone());
            return Err(message);
        }

        Ok(save.unwrap().deleted_count)
    }

    pub async fn many_with_session(
        self,
        client: &Client,
        session: &mut ClientSession,
    ) -> Result<u64, String> {
        if let None = self.orm.filter {
            return Err("Specify filter before deleting...".to_string());
        }

        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(&self.orm.collection);
        let query = self.orm.populate_filter();

        let save = collection.delete_many(query).session(session).await;

        if let Err(why) =save {
            let message = format!("{:?}", why.kind);
            info!(target: "db::delete::error","{}",message.clone());
            return Err(message);
        }

        Ok(save.unwrap().deleted_count)
    }
    pub fn show_merging(self) -> (Vec<Document>, Vec<Document>) {
        self.orm.populate_pipeline()
    }
}
