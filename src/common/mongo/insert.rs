use bson::Document;
use bson::oid::ObjectId;
use log::info;
use mongodb::{Client, ClientSession, Collection};
use serde::{Deserialize, Serialize};
use crate::common::mongo::DB;
use crate::common::orm::get_db_name;
use crate::common::orm::orm::Orm;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Insert {
    orm:  DB,
}

impl Insert {
    pub fn from(from: &str) -> Self {
        Insert {
            orm: DB::get(from),
        }
    }

    pub async fn one_with_session<T: Serialize>(
        self,
        data: T,
        client: &Client,
        session: &mut ClientSession,
    ) -> Result<ObjectId, String> {
        //info!(target: "db::insert","Starting insert data..");
        if self.orm.collection.is_empty() {
            info!(target: "db:insert::error","Collection name is empty");
            return Err("Specify collection name before deleting...".to_string());
        }
        let doc = bson::to_document(&data);
        if doc.is_err() {
            let err_message = doc.unwrap_err().to_string();
            info!(target: "db::insert::error","{}",err_message.clone());
            return Err(err_message);
        }
        let db = client.database(&get_db_name());
        //getting collection info
        let collection: Collection<Document> = db.collection(&self.orm.collection);

        let save = collection.insert_one(doc.unwrap()).session(session).await;

        if save.is_err() {
            let message = format!("{:?}", save.unwrap_err().kind);
            info!(target: "db::insert::error","{}",message.clone());
            return Err(message);
        }

        //info!(target: "db::insert::ok","Successfully inserted data!");
        Ok(save
            .unwrap()
            .inserted_id
            .as_object_id()
            .unwrap_or(ObjectId::new()))
    }

    pub async fn one<T: Serialize>(self, data: T, client: &Client) -> Result<ObjectId, String> {
        //info!(target: "db::insert","Starting insert data..");
        if self.orm.collection.is_empty() {
            info!(target: "db:insert::error","Collection name is empty");
            return Err("Specify collection name before deleting...".to_string());
        }
        let doc = bson::to_document(&data);
        if doc.is_err() {
            let err_message = doc.unwrap_err().to_string();
            info!(target: "db::insert::error","{}",err_message.clone());
            return Err(err_message);
        }
        let db = client.database(&get_db_name());
        //getting collection info
        let collection: Collection<Document> = db.collection(&self.orm.collection);

        let save = collection.insert_one(doc.unwrap()).await;

        if save.is_err() {
            let message = format!("{:?}", save.unwrap_err().kind);
            info!(target: "db::insert::error","{}",message.clone());
            return Err(message);
        }

        //info!(target: "db::insert::ok","Successfully inserted data!");
        Ok(save
            .unwrap()
            .inserted_id
            .as_object_id()
            .unwrap_or(ObjectId::new()))
    }

    pub async fn many<T: Serialize>(
        self,
        data: Vec<T>,
        client: &Client,
    ) -> Result<Vec<ObjectId>, String> {
        // info!(target: "db:insert","Starting insert batch data..");
        if self.orm.collection.is_empty() {
            info!(target: "db:insert::error","Collection name is empty");
            return Err("Specify collection name before deleting...".to_string());
        }
        let mut docs = Vec::new();
        data.iter().for_each(|t| {
            match bson::to_document(t) {
                Ok(doc) => {
                    docs.push(doc);
                }
                Err(_) => {}
            };
        });

        let db = client.database(&get_db_name());
        //getting collection info
        let collection: Collection<Document> = db.collection(&self.orm.collection);

        let save = collection.insert_many(docs).await;

        if save.is_err() {
            let message = format!("{:?}", save.unwrap_err().kind);
            info!(target: "db::insert::error","{}",message.clone());
            return Err(message);
        }
        let save = save
            .unwrap()
            .inserted_ids
            .into_iter()
            .map(|(_, id)| id.as_object_id().unwrap_or(ObjectId::new()))
            .collect::<Vec<ObjectId>>();

        // info!(target: "db::insert::ok","data saved");
        Ok(save)
    }

    pub async fn many_with_session<T: Serialize>(
        self,
        data: Vec<T>,
        client: &Client,
        session: &mut ClientSession,
    ) -> Result<Vec<ObjectId>, String> {
        // info!(target: "db:insert","Starting insert batch data..");
        if self.orm.collection.is_empty() {
            info!(target: "db:insert::error","Collection name is empty");
            return Err("Specify collection name before deleting...".to_string());
        }
        let mut docs = Vec::new();
        data.iter().for_each(|t| {
            match bson::to_document(t) {
                Ok(doc) => {
                    docs.push(doc);
                }
                Err(e) => {
                    info!(target: "db:insert::error","{:?}",e);
                }
            };
        });

        let db = client.database(&get_db_name());
        //getting collection info
        let collection: Collection<Document> = db.collection(&self.orm.collection);

        let save = collection.insert_many(docs).session(session).await;

        if save.is_err() {
            let message = format!("{:?}", save.unwrap_err().kind);
            info!(target: "db::insert::error","{}",message.clone());
            return Err(message);
        }
        let save = save
            .unwrap()
            .inserted_ids
            .into_iter()
            .map(|(_, id)| id.as_object_id().unwrap_or(ObjectId::new()))
            .collect::<Vec<ObjectId>>();

        // info!(target: "db::insert::ok","data saved");
        Ok(save)
    }

    pub fn show_merging(self) -> (Vec<Document>,Vec<Document>) {
        self.orm.populate_pipeline()
    }
}
