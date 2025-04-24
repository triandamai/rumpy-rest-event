use crate::common::api_response::PagingResponse;
use crate::common::mongo::delete::Delete;
use crate::common::mongo::filter::{Filter, FilterGroup, search};
use crate::common::mongo::insert::Insert;
use crate::common::mongo::lookup::Lookup;
use crate::common::mongo::update::Update;
use crate::common::mongo::upsert::Upsert;
use bson::{Bson, Document, doc};
use log::info;
use mongodb::{Client, Collection};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio_stream::StreamExt;

use super::env_config::EnvConfig;

pub mod delete;
pub mod filter;
pub mod insert;
pub mod lookup;
pub mod update;
pub mod upsert;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DB {
    collection: String,
    filter: Option<Filter>,
    lookups: Vec<Lookup>,
    sort: Option<Document>,
    limit: Option<Document>,
    skip: Option<Document>,
    count: Option<Document>,
}

impl DB {
    pub fn get(collection: &str) -> Self {
        DB {
            collection: collection.to_string(),
            filter: None,
            lookups: Vec::new(),
            sort: None,
            limit: None,
            skip: None,
            count: None,
        }
    }

    pub fn insert(collection: &str) -> Insert {
        Insert::from(collection)
    }

    pub fn update(collection: &str) -> Update {
        Update::from(collection)
    }

    pub fn upsert(collection: &str) -> Upsert {
        Upsert::from(collection)
    }

    pub fn delete(collection: &str) -> Delete {
        Delete::from(collection)
    }

    pub fn text<T: Into<Bson>>(mut self, value: T) -> Self {
        let s = search("$search", value);
        if let Some(filter) = self.filter {
            self.filter = Some(filter.set_text(s));
        } else {
            self.filter = Some(Filter::new_with(s));
        }
        self
    }

    pub fn sort(mut self, sort: Vec<(&str, i32)>) -> Self {
        let mut sort_doc = Document::new();
        let mut doc = self.sort.clone().unwrap_or(Document::new());
        for (col, order) in sort {
            doc.insert(col, order);
        }
        sort_doc.insert("$sort", doc);
        self.sort = Some(sort_doc);
        self
    }

    pub fn filter<T: Into<FilterGroup>>(mut self, filters: Vec<T>) -> Self {
        let mut filter = self.filter.clone().unwrap_or(Filter::empty());
        for group in filters {
            match group.into() {
                FilterGroup::Filter(when) => {
                    filter.add_and(when.clone());
                }
                FilterGroup::Or(when) => {
                    for item in when {
                        filter.add_or(item.clone());
                    }
                }
            }
        }
        self.filter = Some(filter);
        self
    }

    pub fn lookup(mut self, lookups: &[Lookup]) -> Self {
        let mut lookup = self.lookups.clone();
        for l in lookups {
            lookup.push(l.clone());
        }
        self.lookups = lookup;
        self
    }

    pub fn populate_filter(self) -> Document {
        let mut doc = Document::new();
        if let Some(filter) = self.filter {
            for item in filter.and {
                for and in item.keys() {
                    doc.insert(and, item.get(and));
                }
            }

            let mut or = Vec::<Document>::new();
            for item in filter.or {
                or.push(item.clone());
            }
            if !or.is_empty() {
                doc.insert("$or", or);
            }
        }
        doc
    }
    pub fn populate_pipeline(self) -> (Vec<Document>, Vec<Document>) {
        let mut pipeline = Vec::new();
        // let mut unwinds = Vec::new();
        if let Some(filter) = self.filter {
            let mut match_doc = Document::new();
            let mut inside_match_doc = Document::new();
            if let Some(text) = filter.text {
                inside_match_doc.insert("$text", text);
            }
            for item in filter.and {
                for and in item.keys() {
                    inside_match_doc.insert(and, item.get(and));
                }
            }

            let mut or = Vec::<Document>::new();
            for item in filter.or {
                or.push(item.clone());
            }

            if !or.is_empty() {
                inside_match_doc.insert("$or", or);
            }
            match_doc.insert("$match", inside_match_doc);
            pipeline.push(match_doc);
        }
        let mut count_all = pipeline.clone();
        for lookup in self.lookups {
            pipeline.push(lookup.doc.clone());
            if let Some(unwind) = lookup.unwind {
                pipeline.push(unwind);
            }
            if let Some(set) = lookup.set {
                pipeline.push(set);
            }
        }

        if let Some(sort) = self.sort {
            pipeline.push(sort);
        }
        // for unwind in unwinds {
        //     pipeline.push(unwind);
        // }

        if let Some(count) = self.count {
            count_all.push(count);
        }
        if let Some(limit) = self.limit {
            pipeline.push(limit);
        }

        if let Some(skip) = self.skip {
            pipeline.push(skip);
        }

        (pipeline, count_all)
    }

    pub async fn get_one<T: DeserializeOwned + Debug>(
        mut self,
        client: &Client,
    ) -> Result<T, String> {
        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(&self.collection);
        let mut limit = Document::new();
        limit.insert("$limit", 1);
        self.limit = Some(limit);

        let (pipeline, _) = self.populate_pipeline();
        // info!(target: "db::get::query","{:?}",pipeline.clone());
        let data = collection.aggregate(pipeline).await;
        if data.is_err() {
            let message = format!("{:?}", data.err().unwrap().kind);
            info!(target: "db::get::error","{}",message.clone());
            return Err(message);
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

    pub async fn get_all<T: DeserializeOwned + Debug>(
        self,
        client: &Client,
    ) -> Result<Vec<T>, String> {
        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(&self.collection);

        let (pipeline, _) = self.populate_pipeline();
        let data = collection.aggregate(pipeline).await;
        if data.is_err() {
            let message = format!("{:?}", data.err().unwrap().kind);
            info!(target: "db::get::error","{}",message.clone());
            return Err(message);
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

            info!(target:"db::get::error:","extract {:?}",item);
            if transform.is_ok() {
                // info!(target: "db::get::ok","extracted success");
                result.push(transform.unwrap());
            } else {
                let err_message = transform.unwrap_err().to_string();
                info!(target:"db::get::error:","extract {:?}",err_message);
            }
        }
        // info!(target: "db::get::ok","data found");
        Ok(result)
    }

    pub async fn get_per_page<T: DeserializeOwned + Debug>(
        mut self,
        page: i64,
        size: i64,
        client: &Client,
    ) -> Result<PagingResponse<T>, String> {
        // info!(target: "db::get","starting get pagination...");
        //getting collection info
        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(&self.collection);

        self.count = Some(doc! {"$count": "total_items"});
        let (limit, skip) = if page > 1 {
            (page.clone() * size.clone(), (page - 1) * size.clone())
        } else {
            (size.clone(), 0)
        };

        let mut limit_doc = Document::new();
        limit_doc.insert("$limit", limit);
        self.limit = Some(limit_doc);

        let mut skip_doc = Document::new();
        skip_doc.insert("$skip", skip);
        self.skip = Some(skip_doc);

        //prepare query
        let (query, query_count) = self.populate_pipeline();

        // info!(target: "db::get","{:?}",query);
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
            let message = format!("{:?}", data.err().unwrap().kind);
            info!(target: "db::get::error","{}",message.clone());
            return Err(message);
        }
        let mut result: Vec<T> = Vec::new();
        let csr = data.unwrap();
        let extract = csr.collect::<Result<Vec<Document>, _>>().await;

        if extract.is_err() {
            let err_message = extract.unwrap_err().to_string();
            info!(target: "db::get::error","extract {}",err_message.clone());

            return Err(err_message);
        }
        let extract = extract.unwrap();

        for item in extract {
            let tr = bson::from_document::<T>(item.clone());
            if tr.is_ok() {
                result.push(tr.unwrap());
            } else {
                let err_message = tr.unwrap_err();
                info!(target:"db::get::error","doc {:?}",item);
                info!(target:"db::get::error","{:?}",err_message.clone());
            }
        }

        let total_pages = (total_items.clone() as f64 / skip as f64).ceil() as u32;
        //info!(target: "db::get::ok","total pages {}", total_items.clone());
        Ok(PagingResponse {
            total_items: total_items as i64,
            total_pages: total_pages as i64,
            page,
            size,
            items: result,
        })
    }

    pub async fn find_one<T: DeserializeOwned>(
        self,
        query: Document,
        client: &Client,
    ) -> Option<T> {
        //info!(target: "db::get","find one..");
        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(&self.collection);
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
        let db = client.database(&get_db_name());
        let collection: Collection<Document> = db.collection(&self.collection);
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
            let extract = bson::from_document::<T>(item.clone());

            match extract {
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
}

pub fn get_db_name() -> String {
    let env = EnvConfig::init();
    format!("RUMPY-EVENT-{}", env.mode).to_lowercase()
}
