use crate::common::api_response::PagingResponse;
use crate::common::orm::orm::{create_count_field, create_limit_field, create_skip_field, Orm};
use crate::common::orm::DB_NAME;
use crate::common::utils::create_object_id_option;
use bson::oid::ObjectId;
use bson::{doc, Document};
use log::info;
use mongodb::{Client, Collection};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio_stream::StreamExt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Get {
    orm: Orm,
}

impl Get {
    pub fn from(from: &str) -> Self {
        Get {
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

    pub async fn one<T: DeserializeOwned + Debug>(mut self, client: &Client) -> Result<T, String> {
        //info!(target: "db::get","starting get one..");
        let db = client.database(DB_NAME);
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        self.orm.limit = Some(create_limit_field(1));
        let pipeline = self.orm.merge_field(true);
        let data = collection.aggregate(pipeline).await;

        if data.is_err() {
            let err_message = data.unwrap_err();
            info!(target: "db::get::error","{}",err_message.clone().to_string());
            return Err(err_message.to_string());
        }
        let mut data = data.unwrap();
        let next = data.next().await;
        if next.is_none() {
            //info!(target: "db::get::not_found","No data found");
            return Err("Tidak dapat menemukan data".to_string());
        }
        let next = next.unwrap();
        if next.is_err() {
            return Err(next.unwrap_err().to_string());
        }
        let next = next.unwrap();

        let transform = bson::from_document::<T>(next);
        if transform.is_err() {
            let err_message = transform.unwrap_err().to_string();
            info!(target: "db::get::error","{}",err_message.clone());

            return Err(format!("{:?}", err_message));
        }

        // info!(target: "db::get::ok","data found");
        Ok(transform.unwrap())
    }

    pub async fn all<T: DeserializeOwned + Debug>(self, client: &Client) -> Result<Vec<T>, String> {
        //info!(target: "db::get","starting get ALL..");
        let db = client.database(DB_NAME);
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let pipeline = self.orm.merge_field_all(true);
        let data = collection.aggregate(pipeline).await;

        if data.is_err() {
            let err_message = data.unwrap_err().to_string();
            info!(target: "db::get::error 1","{}",err_message.clone());
            return Err(err_message);
        }
        let mut result: Vec<T> = Vec::new();
        let csr = data.unwrap();
        let extract = csr.collect::<Result<Vec<Document>, _>>().await;

        if extract.is_err() {
            let err_message = extract.unwrap_err().to_string();
            info!(target: "db::get::error","{}",err_message.clone());
            return Err(err_message);
        }
        let extract = extract.unwrap();

        for item in extract {
            let transform = bson::from_document::<T>(item.clone());

            // info!(target:"db::get::error:","extract {:?}",item);
            if transform.is_ok() {
                result.push(transform.unwrap());
            } else {
                let err_message = transform.unwrap_err().to_string();
                info!(target:"db::get::error:","extract {:?}",err_message);
            }
        }
        // info!(target: "db::get::ok","data found");
        Ok(result)
    }

    pub async fn pageable<T: DeserializeOwned + Debug>(
        mut self,
        page: i64,
        size: i64,
        client: &Client,
    ) -> Result<PagingResponse<T>, String> {
        info!(target: "db::get","starting get pagination...");
        //getting collection info
        let db = client.database(DB_NAME);
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());

        self.orm.count = Some(create_count_field());
        self.orm.limit = Some(create_limit_field(size));

        let final_skip = if page < 1 {
            size
        } else {
            (page.clone() - 1) * size.clone()
        };

        self.orm.skip = Some(create_skip_field(final_skip));
        //prepare query
        let (query, query_count) = self.orm.merge_field_pageable(true);

        // info!(target: "db::get","{:?}",query_count);
        let get_count = collection.aggregate(query_count).await;

        let total_items = match get_count {
            Ok(mut value) => StreamExt::try_next(&mut value).await.map_or_else(
                |_| None,
                |document| {
                    document
                        .map_or_else(|| None, |doc| Some(doc.get_i32("total_items").unwrap_or(0)))
                },
            ),
            Err(_) => None,
        }
        .unwrap_or(0);

        let data = collection.aggregate(query).await;

        if data.is_err() {
            let err_message = data.unwrap_err().to_string();
            info!(target: "db::get::error","{}",err_message.clone());
            return Err(err_message);
        }
        let mut result: Vec<T> = Vec::new();
        let csr = data.unwrap();
        let extract = csr.collect::<Result<Vec<Document>, _>>().await;

        if extract.is_err() {
            let err_message = extract.unwrap_err().to_string();
            info!(target: "db::get::error","{}",err_message.clone());

            return Err(err_message);
        }
        let extract = extract.unwrap();

        for item in extract {
            let tr = bson::from_document::<T>(item.clone());

            if tr.is_ok() {
                result.push(tr.unwrap());
            } else {
                let err_message = tr.unwrap_err().to_string();
                info!(target:"db::get::error","extract from doc {:?}",err_message);
            }
        }

        let total_pages = (total_items.clone() as f64 / size as f64).ceil() as u32;
        //info!(target: "db::get::ok","total pages {}", total_items.clone());
        Ok(PagingResponse {
            total_items: total_items as i64,
            total_pages: total_pages as i64,
            page: page,
            size: size,
            items: result,
        })
    }

    pub async fn find_one<T: DeserializeOwned>(
        self,
        query: Document,
        client: &Client,
    ) -> Option<T> {
        //info!(target: "db::get","find one..");
        let db = client.database(DB_NAME);
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        collection.find_one(query).await.map_or_else(
            |_| None,
            |value| {
                if value.is_none() {
                    return None;
                }
                let value = value.unwrap();
                let data = bson::from_document::<T>(value);
                if data.is_err() {
                    return None;
                }
                Some(data.unwrap())
            },
        )
    }

    pub async fn find_many<T: DeserializeOwned>(self, query: Document, client: &Client) -> Vec<T> {
        let db = client.database(DB_NAME);
        let collection: Collection<Document> = db.collection(self.orm.collection_name.as_str());
        let find = collection.find(query).await;
        if find.is_err() {
            return Vec::new();
        }
        let find_as_cursor = find.unwrap();
        let next = find_as_cursor.collect::<Result<Vec<Document>, _>>().await;
        if next.is_err() {
            return Vec::new();
        }
        let mut result: Vec<T> = Vec::new();
        let next = next.unwrap();
        for item in next {
            let tr = bson::from_document::<T>(item.clone());

            match tr {
                Ok(value) => {
                    result.push(value);
                }
                Err(e) => {
                    info!(target:"extract from doc:","{:?}",e);
                }
            }
        }
        result
    }

    //query
    pub fn filter_bool(mut self, column: &str, operator: Option<&str>, value: bool) -> Self {
        let orm = self.orm.filter_bool(column, operator, value.clone());
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

    pub fn join_one(
        mut self,
        collection: &str,
        from_field: &str,
        foreign_field: &str,
        alias: &str,
    ) -> Self {
        let orm = self
            .orm
            .join_one(collection, from_field, foreign_field, alias);
        self.orm = orm;
        self
    }
    pub fn join_many(
        mut self,
        collection: &str,
        from_field: &str,
        foreign_field: &str,
        alias: &str,
    ) -> Self {
        let orm = self
            .orm
            .join_many(collection, from_field, foreign_field, alias);
        self.orm = orm;
        self
    }

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

    pub fn group_by_asc(mut self, column: &str) -> Self {
        let orm = self.orm.group_by_asc(column);
        self.orm = orm;
        self
    }

    pub fn group_by_desc(mut self, column: &str) -> Self {
        let orm = self.orm.group_by_desc(column);
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

    pub fn filter_object_id_as_str(mut self, column: &str, value: &str) -> Self {
        let object_id = create_object_id_option(value);

        let orm = match object_id {
            None => self.orm.filter_string(column, Some("$eq"), value),
            Some(v) => self.orm.filter_object_id_with_equal(column, &v),
        };
        self.orm = orm;
        self
    }

    pub fn show_merging(self) -> (Vec<Document>, Vec<Document>) {
        self.orm.merge_field_pageable(true)
    }
}
