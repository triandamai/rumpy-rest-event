use crate::common::orm::delete::Delete;
use crate::common::orm::get::Get;
use crate::common::orm::insert::Insert;
use crate::common::orm::replace::Replace;
use crate::common::orm::update::Update;
use bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

pub fn create_lookup_field(
    target: &str,
    from_field: &str,
    foreign_field: &str,
    alias: &str,
) -> Document {
    doc! {
        "$lookup":{
            "from":target,
            "localField":from_field,
            "foreignField":foreign_field,
            "as":alias
        }
    }
}
pub fn create_field_text_search(text: String) -> Document {
    doc! {"$match":{"$text":{"$search":text}}}
}

pub fn create_unwind_field(column: &str) -> Document {
    doc! {
        "$unwind":{
            "path":column,
            "preserveNullAndEmptyArrays":true
        }
    }
}

pub fn create_limit_field(limit: i64) -> Document {
    doc! {"$limit":limit}
}
pub fn create_skip_field(skip: i64) -> Document {
    doc! {"$skip":skip}
}

pub fn create_sort_desc_field(column: &str) -> Document {
    doc! { column: -1 /*Sort by created_at in descending order*/}
}

pub fn create_sort_asc_field(column: &str) -> Document {
    doc! {column: 1 /* Sort by created_at in descending order*/}
}

pub fn create_count_field() -> Document {
    doc! {"$count": "total_items"}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Filter {
    operator: String,
    filter: Vec<Document>,
}

impl Filter {
    pub fn new(operator: String, filter: Vec<Document>) -> Self {
        Filter { operator, filter }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Orm {
    pub collection_name: String,
    pub filter: Vec<Document>,
    pub filters_group: HashMap<String, Filter>,
    pub current_filter: Option<String>,
    pub lookup: Vec<Document>,
    pub unwind: Vec<Document>,
    pub sort: Vec<Document>,
    pub count: Option<Document>,
    pub skip: Option<Document>,
    pub limit: Option<Document>,
}

impl Orm {
    pub fn get(from: &str) -> Get {
        Get::from(from)
    }

    pub fn update(from: &str) -> Update {
        Update::from(from)
    }

    pub fn insert(from: &str) -> Insert {
        Insert::from(from)
    }

    pub fn replace(from: &str) -> Replace {
        Replace::from(from)
    }
    pub fn delete(from: &str) -> Delete {
        Delete::from(from)
    }

    pub fn join_one(
        mut self,
        collection: &str,
        from_field: &str,
        foreign_field: &str,
        alias: &str,
    ) -> Self {
        let doc = create_lookup_field(collection, from_field, foreign_field, alias);
        let unwind = create_unwind_field(format!("${}", alias).as_str());
        self.lookup.push(doc);
        self.unwind.push(unwind);
        self
    }
    pub fn join_many(
        mut self,
        collection: &str,
        from_field: &str,
        foreign_field: &str,
        alias: &str,
    ) -> Self {
        let doc = create_lookup_field(collection, from_field, foreign_field, alias);
        self.lookup.push(doc);
        self
    }

    pub fn group_by_asc(mut self, column: &str) -> Self {
        let mut sort = self.sort;
        sort.push(create_sort_asc_field(column));
        self.sort = sort;
        self
    }

    pub fn group_by_desc(mut self, column: &str) -> Self {
        let mut sort = self.sort;
        sort.push(create_sort_desc_field(column));
        self.sort = sort;
        self
    }

    pub fn or(mut self) -> Self {
        let key = format!("{}", self.filters_group.len() + 1);
        self.current_filter = Some(key.clone());
        self.filters_group
            .insert(key, Filter::new("$or".to_string(), Vec::new()));
        self
    }
    pub fn text(mut self) -> Self {
        let key = format!("{}", self.filters_group.len() + 1);
        self.current_filter = Some(key.clone());
        self.filters_group
            .insert(key, Filter::new("$text".to_string(), Vec::new()));
        self
    }

    pub fn and(mut self) -> Self {
        let key = format!("{}", self.filters_group.len() + 1);
        self.current_filter = Some(key.clone());
        self.filters_group
            .insert(key, Filter::new("$and".to_string(), Vec::new()));
        self
    }

    pub fn filter_bool(mut self, column: &str, operator: Option<&str>, value: bool) -> Self {
        let mut doc = Document::new();
        if operator.is_none() {
            doc.insert(column, value);
        } else {
            let mut eq = Document::new();
            eq.insert(operator.unwrap(), value);
            doc.insert(column, eq);
        }

        if self.current_filter.is_none() {
            self.filter.push(doc);
        } else {
            let map = self.current_filter.clone().unwrap();
            let hp = self.filters_group.get(&map.clone());
            match hp {
                None => {}
                Some(filter) => {
                    let mut f = filter.clone();
                    f.filter.push(doc);
                    self.filters_group.insert(map, f);
                }
            }
        }
        self
    }
    pub fn filter_array<T: Serialize>(
        mut self,
        column: &str,
        operator: Option<&str>,
        value: Vec<T>,
    ) -> Self {
        let mut doc = Document::new();
        let value_as_doc = bson::to_document(&value).unwrap_or(Document::new());
        if operator.is_none() {
            doc.insert(column, value_as_doc);
        } else {
            let mut eq = Document::new();
            eq.insert(operator.unwrap(), value_as_doc);
            doc.insert(column, eq);
        }

        if self.current_filter.is_none() {
            self.filter.push(doc);
        } else {
            let map = self.current_filter.clone().unwrap();
            let hp = self.filters_group.get(&map.clone());
            match hp {
                None => {}
                Some(filter) => {
                    let mut f = filter.clone();
                    f.filter.push(doc);
                    self.filters_group.insert(map, f);
                }
            }
        }
        self
    }

    pub fn filter_number(mut self, column: &str, operator: Option<&str>, value: i64) -> Self {
        let mut doc = Document::new();
        if operator.is_none() {
            doc.insert(column, value);
        } else {
            let mut eq = Document::new();
            eq.insert(operator.unwrap(), value);
            doc.insert(column, eq);
        }

        if self.current_filter.is_none() {
            self.filter.push(doc);
        } else {
            let map = self.current_filter.clone().unwrap();
            let hp = self.filters_group.get(&map.clone());
            match hp {
                None => {}
                Some(filter) => {
                    let mut f = filter.clone();
                    f.filter.push(doc);
                    self.filters_group.insert(map, f);
                }
            }
        }
        self
    }
    pub fn filter_string(mut self, column: &str, operator: Option<&str>, value: &str) -> Self {
        let mut doc = Document::new();
        if operator.is_none() {
            doc.insert(column, value);
        } else {
            let mut eq = Document::new();
            eq.insert(operator.unwrap(), value);
            doc.insert(column, eq);
        }

        if self.current_filter.is_none() {
            self.filter.push(doc);
        } else {
            let map = self.current_filter.clone().unwrap();
            let hp = self.filters_group.get(&map.clone());
            match hp {
                None => {}
                Some(filter) => {
                    let mut f = filter.clone();
                    f.filter.push(doc);
                    self.filters_group.insert(map, f);
                }
            }
        }
        self
    }

    pub fn filter_search_string(
        mut self,
        column: &str,
        operator: Option<&str>,
        value: &str,
    ) -> Self {
        let mut doc = Document::new();
        if operator.is_none() {
            doc.insert(column, value);
        } else {
            let mut eq = Document::new();
            eq.insert(operator.unwrap(), value);
            doc.insert(column, eq);
        }

        if self.current_filter.is_none() {
            self.filter.push(doc);
        } else {
            let map = self.current_filter.clone().unwrap();
            let hp = self.filters_group.get(&map.clone());
            match hp {
                None => {}
                Some(filter) => {
                    let mut f = filter.clone();
                    f.filter.push(doc);
                    self.filters_group.insert(map, f);
                }
            }
        }
        self
    }
    pub fn filter_object_id(mut self, column: &str, value: &ObjectId) -> Self {
        let mut doc = Document::new();

        doc.insert(column, value);

        if self.current_filter.is_none() {
            self.filter.push(doc);
        } else {
            let map = self.current_filter.clone().unwrap();
            let hp = self.filters_group.get(&map.clone());
            match hp {
                None => {}
                Some(filter) => {
                    let mut f = filter.clone();
                    f.filter.push(doc);
                    self.filters_group.insert(map, f);
                }
            }
        }
        self
    }

    pub fn filter_object_id_with_equal(mut self, column: &str, value: &ObjectId) -> Self {
        let mut doc = Document::new();

        doc.insert(column, doc! {"$eq":value});

        if self.current_filter.is_none() {
            self.filter.push(doc);
        } else {
            let map = self.current_filter.clone().unwrap();
            let hp = self.filters_group.get(&map.clone());
            match hp {
                None => {}
                Some(filter) => {
                    let mut f = filter.clone();
                    f.filter.push(doc);
                    self.filters_group.insert(map, f);
                }
            }
        }
        self
    }

    pub fn merge_field_all(self, is_aggregate: bool) -> Vec<Document> {
        let mut result: Vec<Document> = Vec::new();

        if self.filters_group.len() > 0 {
            let mut parent: Document = Document::new();
            let mut upper_filter: Document = Document::new();
            for (_, filter) in self.filters_group {
                //if opr = $text use object instead
                if filter.operator == "$text" {
                    if filter.filter.len() > 1 {
                        let mut result_child = Document::new();
                        for child in filter.filter {
                            for child_key in child.keys() {
                                result_child.insert(child_key, child.get(child_key));
                            }
                        }
                        upper_filter.insert(filter.operator.clone(), result_child);
                    }
                } else {
                    if filter.filter.len() > 1 {
                        let mut result_child: Vec<Document> = Vec::new();

                        for child in filter.filter {
                            result_child.push(child);
                        }
                        upper_filter.insert(filter.operator, result_child);
                    }
                }
            }

            if !upper_filter.is_empty() {
                if is_aggregate {
                    parent.insert("$match", upper_filter);
                    result.push(parent.clone());
                } else {
                    result.push(upper_filter.clone());
                }
            }
        } else {
            let mut parent = Document::new();
            let mut result2: Vec<Document> = Vec::new();
            for f in self.filter {
                if is_aggregate {
                    result2.push(f.clone());
                } else {
                    result.push(f.clone());
                }
            }
            if is_aggregate {
                if result2.len() > 1 {
                    parent.insert("$match", result2);
                } else {
                    if result2.get(0).is_some() {
                        let v = result2.get(0).unwrap();
                        parent.insert("$match", v);
                    }
                }
                if !parent.is_empty() {
                    result.push(parent.clone());
                }
            }
        }

        if self.sort.len() > 0 {
            let mut sort_doc = Document::new();
            let mut doc = Document::new();
            for child in self.sort {
                for key in child.keys() {
                    doc.insert(key, child.get(key));
                }
            }
            sort_doc.insert("$sort", doc);
            result.push(sort_doc);
        }

        for lookup in self.lookup {
            result.push(lookup);
        }

        for unwind in self.unwind {
            result.push(unwind);
        }

        if self.count.is_some() {
            result.push(self.count.unwrap());
        }

        if self.skip.is_some() {
            result.push(self.skip.unwrap());
        }
        // NO NEED LIMIT
        // if self.limit.is_some() {
        //     result.push(self.limit.unwrap());
        // }
        result
    }

    pub fn merge_field_pageable(self, is_aggregate: bool) -> (Vec<Document>, Vec<Document>) {
        let mut result_count: Vec<Document> = Vec::new();
        let mut result: Vec<Document> = Vec::new();

        if self.filters_group.len() > 0 {
            let mut parent: Document = Document::new();
            let mut upper_filter: Document = Document::new();
            for (_, filter) in self.filters_group {
                //if opr = $text use object instead
                if filter.operator == "$text" {
                    if filter.filter.len() > 1 {
                        let mut result_child = Document::new();
                        for child in filter.filter {
                            for child_key in child.keys() {
                                result_child.insert(child_key, child.get(child_key));
                            }
                        }
                        upper_filter.insert(filter.operator.clone(), result_child);
                    }
                } else {
                    if filter.filter.len() > 1 {
                        let mut result_child: Vec<Document> = Vec::new();

                        for child in filter.filter {
                            result_child.push(child);
                        }
                        upper_filter.insert(filter.operator, result_child);
                    }
                }
            }

            if !upper_filter.is_empty() {
                if is_aggregate {
                    parent.insert("$match", upper_filter);
                    result.push(parent.clone());
                    result_count.push(parent.clone());
                } else {
                    result.push(upper_filter.clone());
                    result_count.push(parent.clone());
                }
            }
        } else {
            let mut parent = Document::new();
            let mut result2: Vec<Document> = Vec::new();
            for f in self.filter {
                if is_aggregate {
                    result2.push(f.clone());
                } else {
                    result.push(f.clone());
                }
            }
            if is_aggregate {
                if result2.len() > 1 {
                    parent.insert("$match", result2);
                } else {
                    if result2.get(0).is_some() {
                        let v = result2.get(0).unwrap();
                        parent.insert("$match", v);
                    }
                }
                if !parent.is_empty() {
                    result.push(parent.clone());
                    result_count.push(parent.clone());
                }
            }
        }
        if self.sort.len() > 0 {
            let mut sort_doc = Document::new();
            let mut doc = Document::new();
            for child in self.sort {
                for key in child.keys() {
                    doc.insert(key, child.get(key));
                }
            }
            sort_doc.insert("$sort", doc);
            result.push(sort_doc);
        }

        for lookup in self.lookup {
            result.push(lookup.clone());
            result_count.push(lookup);
        }

        for unwind in self.unwind {
            result.push(unwind.clone());
            result_count.push(unwind);
        }

        if self.count.is_some() {
            result_count.push(self.count.unwrap());
        }
        if self.limit.is_some() {
            result.push(self.limit.unwrap())
        }
        if self.skip.is_some() {
            result.push(self.skip.unwrap())
        }

        (result, result_count)
    }
    pub fn get_filter_as_doc(self) -> Document {
        if self.filters_group.len() > 0 {
            let mut upper_filter: Document = Document::new();
            for (_, filter) in self.filters_group {
                let mut result_child: Vec<Document> = Vec::new();
                for child in filter.filter {
                    result_child.push(child);
                }
                upper_filter.insert(filter.operator, result_child);
            }
            upper_filter
        } else {
            self.filter.first().unwrap().clone()
        }
    }

    pub fn merge_field(self, is_aggregate: bool) -> Vec<Document> {
        let mut result: Vec<Document> = Vec::new();

        if self.filters_group.len() > 0 {
            let mut parent: Document = Document::new();
            let mut upper_filter: Document = Document::new();
            for (_, filter) in self.filters_group {
                if filter.filter.len() > 1 {
                    let mut result_child: Vec<Document> = Vec::new();
                    for child in filter.filter {
                        result_child.push(child);
                    }
                    upper_filter.insert(filter.operator, result_child);
                }
            }
            if !upper_filter.is_empty() {
                if is_aggregate {
                    parent.insert("$match", upper_filter);
                    result.push(parent.clone());
                } else {
                    result.push(upper_filter.clone());
                }
            }
        } else {
            let mut parent = Document::new();
            let mut result2: Vec<Document> = Vec::new();
            for f in self.filter {
                if is_aggregate {
                    result2.push(f.clone());
                } else {
                    result.push(f.clone());
                }
            }
            if is_aggregate {
                if result2.len() > 1 {
                    parent.insert("$match", result2);
                } else {
                    parent.insert("$match", result2.get(0).unwrap());
                }
                if !parent.is_empty() {
                    result.push(parent.clone());
                }
            }
        }

        if self.sort.len() > 0 {
            let mut sort_doc = Document::new();
            let mut doc = Document::new();
            for child in self.sort {
                for key in child.keys() {
                    doc.insert(key, child.get(key));
                }
            }
            sort_doc.insert("$sort", doc);
            result.push(sort_doc);
        }
        for lookup in self.lookup {
            result.push(lookup);
        }

        for unwind in self.unwind {
            result.push(unwind);
        }

        if self.count.is_some() {
            result.push(self.count.unwrap());
        }

        if self.skip.is_some() {
            result.push(self.skip.unwrap());
        }
        if self.limit.is_some() {
            result.push(self.limit.unwrap());
        }
        result
    }

    pub fn show_merging(self) -> Vec<Document> {
        self.merge_field(true)
    }
}
