// use bson::{doc, Bson, Document};
// use log::info;
// use serde::{de::DeserializeOwned, Deserialize, Serialize};
// use serde_helpers::SerializeJson;
// use sqlx::{FromRow, Postgres};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct DB {
//     pub table_name: String,
//     pub query: String,
//     pub args: Vec<Bson>,
// }

// impl DB {
//     pub fn from(table: &str) -> Self {
//         DB {
//             table_name: table.to_string(),
//             query: "".to_string(),
//             args: vec![],
//         }
//     }

//     pub async fn insert<T: DeserializeOwned + Serialize>(
//         self,
//         data: T,
//         pool: &sqlx::Pool<Postgres>,
//     ) -> Result<T, String>
//     where
//         T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
//     {
//         let extract = bson::to_document(&data);
//         if extract.is_err() {
//             return Err(extract.unwrap_err().to_string());
//         }
//         let mut values: String = String::new();
//         let mut params: String = String::new();
//         let mut args: Vec<Bson> = Vec::new();
//         let datas = extract.unwrap();
//         let len = datas.len();
//         let mut idx = 1;
//         for (index, payload) in datas.iter().enumerate() {
//             if payload.1.clone() != Bson::Null {
//                 values += payload.0.as_str();
//                 params += format!("${}", idx).as_str();
//                 if index < len - 1 && len > 0 {
//                     values += ",";
//                     params += ",";
//                 }
//                 args.push(payload.1.clone());
//                 idx += 1;
//             }
//         }
//         let sql_insert = format!(
//             "INSERT INTO {}({}) VALUES({}) RETURNING *;",
//             self.table_name, values, params
//         );
//         info!(target:"db::execute","{}",sql_insert);
//         let mut db = sqlx::query_as::<_, T>(&sql_insert);
//         for arg in args {
//             match arg {
//                 Bson::Double(value) => {
//                     info!(target:"db::execute::d","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::String(value) => {
//                     info!(target:"db::execute::s","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Array(bsons) => {
//                     info!(target:"db::execute::b","{:?}",bsons.clone());
//                     let data = bsons
//                         .iter()
//                         .map(|v| v.to_json_string().unwrap_or(String::new()))
//                         .collect::<Vec<String>>();
//                     db = db.bind(data);
//                 }
//                 Bson::Boolean(value) => {
//                     info!(target:"db::execute::bo","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Null => {
//                     info!(target:"db::execute::null","");
//                     db = db.bind("null");
//                 }
//                 Bson::RegularExpression(regex) => {
//                     info!(target:"db::execute::reg","{}",regex);
//                     db = db.bind(regex.pattern);
//                 }
//                 Bson::Int32(value) => {
//                     info!(target:"db::execute::i32","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Int64(value) => {
//                     info!(target:"db::execute::i64","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Timestamp(timestamp) => {
//                     info!(target:"db::execute::tim","{}",timestamp);
//                     db = db.bind(timestamp.time as i64);
//                 }
//                 Bson::ObjectId(object_id) => {
//                     info!(target:"db::execute::obj","{}",object_id);
//                     db = db.bind(object_id.to_string());
//                 }
//                 Bson::DateTime(date_time) => {
//                     info!(target:"db::execute::dtm","{}",date_time);
//                     db = db.bind(date_time.timestamp_millis());
//                 }
//                 Bson::Decimal128(decimal128) => {
//                     info!(target:"db::execute::dec","{}",decimal128);
//                     db = db.bind(decimal128.bytes());
//                 }
//                 _ => {}
//             };
//         }
//         let execute = db.fetch_optional(pool).await;
//         if execute.is_err() {
//             return Err(format!("{:?}", execute.err()));
//         }
//         let execute = execute.unwrap();
//         if execute.is_none() {
//             return Err("NOT RETURNING".to_string());
//         }
//         Ok(execute.unwrap())
//     }

//     pub async fn insert_raw<T: DeserializeOwned + Serialize>(
//         self,
//         data: Document,
//         pool: &sqlx::Pool<Postgres>,
//     ) -> Result<T, String>
//     where
//         T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
//     {
//         let extract = bson::to_document(&data);
//         if extract.is_err() {
//             return Err(extract.unwrap_err().to_string());
//         }
//         let mut values: String = String::new();
//         let mut params: String = String::new();
//         let mut args: Vec<Bson> = Vec::new();
//         let datas = extract.unwrap();
//         let len = datas.len();
//         let mut idx = 1;
//         for (index, payload) in datas.iter().enumerate() {
//             if payload.1.clone() != Bson::Null {
//                 values += payload.0.as_str();
//                 params += format!("${}", idx).as_str();
//                 if index < len - 1 && len > 0 {
//                     values += ",";
//                     params += ",";
//                 }
//                 args.push(payload.1.clone());
//                 idx += 1;
//             }
//         }
//         let sql_insert = format!(
//             "INSERT INTO {}({}) VALUES({}) RETURNING *;",
//             self.table_name, values, params
//         );
//         info!(target:"db::execute","{}",sql_insert);
//         let mut db = sqlx::query_as::<_, T>(&sql_insert);
//         for arg in args {
//             match arg {
//                 Bson::Double(value) => {
//                     info!(target:"db::execute::d","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::String(value) => {
//                     info!(target:"db::execute::s","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Array(bsons) => {
//                     info!(target:"db::execute::b","{:?}",bsons.clone());
//                     let data = bsons
//                         .iter()
//                         .map(|v| v.to_json_string().unwrap_or(String::new()))
//                         .collect::<Vec<String>>();
//                     db = db.bind(data);
//                 }
//                 Bson::Boolean(value) => {
//                     info!(target:"db::execute::bo","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Null => {
//                     info!(target:"db::execute::null","");
//                     db = db.bind("null");
//                 }
//                 Bson::RegularExpression(regex) => {
//                     info!(target:"db::execute::reg","{}",regex);
//                     db = db.bind(regex.pattern);
//                 }
//                 Bson::Int32(value) => {
//                     info!(target:"db::execute::i32","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Int64(value) => {
//                     info!(target:"db::execute::i64","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Timestamp(timestamp) => {
//                     info!(target:"db::execute::tim","{}",timestamp);
//                     db = db.bind(timestamp.time as i64);
//                 }
//                 Bson::ObjectId(object_id) => {
//                     info!(target:"db::execute::obj","{}",object_id);
//                     db = db.bind(object_id.to_string());
//                 }
//                 Bson::DateTime(date_time) => {
//                     info!(target:"db::execute::dtm","{}",date_time);
//                     db = db.bind(date_time.timestamp_millis());
//                 }
//                 Bson::Decimal128(decimal128) => {
//                     info!(target:"db::execute::dec","{}",decimal128);
//                     db = db.bind(decimal128.bytes());
//                 }
//                 _ => {}
//             };
//         }
//         let execute = db.fetch_optional(pool).await;
//         if execute.is_err() {
//             return Err(format!("{:?}", execute.err()));
//         }
//         let execute = execute.unwrap();
//         if execute.is_none() {
//             return Err("NOT RETURNING".to_string());
//         }
//         Ok(execute.unwrap())
//     }

//     pub async fn update<T: DeserializeOwned + Serialize>(
//         self,
//         data: T,
//         pool: &sqlx::Pool<Postgres>,
//     ) -> Result<T, String>
//     where
//         T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
//     {
//         let extract = bson::to_document(&data);
//         if extract.is_err() {
//             return Err(extract.unwrap_err().to_string());
//         }
//         let mut params: String = String::new();
//         let mut args: Vec<Bson> = Vec::new();
//         let datas = extract.unwrap();
//         let len = datas.len();
//         let mut idx = 1;
//         for (index, payload) in datas.iter().enumerate() {
//             if payload.1.clone() != Bson::Null {
//                 params += format!("{}=${}", payload.0.as_str(), idx).as_str();
//                 if index < len - 1 && len > 0 {
//                     params += ",";
//                 }
//                 args.push(payload.1.clone());
//                 idx += 1;
//             }
//         }
//         let sql_update = format!(
//             "UPDATE {} SET {} {} RETURNING *;",
//             self.table_name, params, self.query
//         );
//         info!(target:"db::execute","{}",sql_update);
//         let mut db = sqlx::query_as::<_, T>(&sql_update);
//         for arg in args {
//             match arg {
//                 Bson::Double(value) => {
//                     info!(target:"db::execute::d","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::String(value) => {
//                     info!(target:"db::execute::s","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Array(bsons) => {
//                     info!(target:"db::execute::b","{:?}",bsons.clone());
//                     let data = bsons
//                         .iter()
//                         .map(|v| v.to_json_string().unwrap_or(String::new()))
//                         .collect::<Vec<String>>();
//                     db = db.bind(data);
//                 }
//                 Bson::Boolean(value) => {
//                     info!(target:"db::execute::bo","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Null => {
//                     info!(target:"db::execute::null","");
//                     db = db.bind("null");
//                 }
//                 Bson::RegularExpression(regex) => {
//                     info!(target:"db::execute::reg","{}",regex);
//                     db = db.bind(regex.pattern);
//                 }
//                 Bson::Int32(value) => {
//                     info!(target:"db::execute::i32","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Int64(value) => {
//                     info!(target:"db::execute::i64","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Timestamp(timestamp) => {
//                     info!(target:"db::execute::tim","{}",timestamp);
//                     db = db.bind(timestamp.time as i64);
//                 }
//                 Bson::ObjectId(object_id) => {
//                     info!(target:"db::execute::obj","{}",object_id);
//                     db = db.bind(object_id.to_string());
//                 }
//                 Bson::DateTime(date_time) => {
//                     info!(target:"db::execute::dtm","{}",date_time);
//                     db = db.bind(date_time.timestamp_millis());
//                 }
//                 Bson::Symbol(_) => {}
//                 Bson::Decimal128(decimal128) => {
//                     info!(target:"db::execute::dec","{}",decimal128);
//                     db = db.bind(decimal128.bytes());
//                 }
//                 _ => {}
//             };
//         }
//         let execute = db.fetch_optional(pool).await;
//         if execute.is_err() {
//             return Err(format!("{:?}", execute.err()));
//         }
//         let execute = execute.unwrap();
//         if execute.is_none() {
//             return Err("NOT RETURNING".to_string());
//         }
//         Ok(execute.unwrap())
//     }

//     pub async fn update_raw<T: DeserializeOwned + Serialize>(
//         self,
//         data: Document,
//         pool: &sqlx::Pool<Postgres>,
//     ) -> Result<T, String>
//     where
//         T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
//     {
//         let mut params: String = String::new();
//         let mut args: Vec<Bson> = Vec::new();
//         let len = data.len();
//         let mut idx = 1;
//         for (index, payload) in data.iter().enumerate() {
//             if payload.1.clone() != Bson::Null {
//                 params += format!("{}=${}", payload.0.as_str(), idx).as_str();
//                 if index < len - 1 && len > 0 {
//                     params += ",";
//                 }
//                 args.push(payload.1.clone());
//                 idx += 1;
//             }
//         }
//         let sql_update = format!(
//             "UPDATE {} SET {} {} RETURNING *;",
//             self.table_name, params, self.query
//         );
//         info!(target:"db::execute","{}",sql_update);
//         let mut db = sqlx::query_as::<_, T>(&sql_update);
//         for arg in args {
//             match arg {
//                 Bson::Double(value) => {
//                     info!(target:"db::execute::d","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::String(value) => {
//                     info!(target:"db::execute::s","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Array(bsons) => {
//                     info!(target:"db::execute::b","{:?}",bsons.clone());
//                     let data = bsons
//                         .iter()
//                         .map(|v| v.to_json_string().unwrap_or(String::new()))
//                         .collect::<Vec<String>>();
//                     db = db.bind(data);
//                 }
//                 Bson::Boolean(value) => {
//                     info!(target:"db::execute::bo","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Null => {
//                     info!(target:"db::execute::null","");
//                     db = db.bind("null");
//                 }
//                 Bson::RegularExpression(regex) => {
//                     info!(target:"db::execute::reg","{}",regex);
//                     db = db.bind(regex.pattern);
//                 }
//                 Bson::Int32(value) => {
//                     info!(target:"db::execute::i32","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Int64(value) => {
//                     info!(target:"db::execute::i64","{}",value);
//                     db = db.bind(value);
//                 }
//                 Bson::Timestamp(timestamp) => {
//                     info!(target:"db::execute::tim","{}",timestamp);
//                     db = db.bind(timestamp.time as i64);
//                 }

//                 Bson::ObjectId(object_id) => {
//                     info!(target:"db::execute::obj","{}",object_id);
//                     db = db.bind(object_id.to_string());
//                 }
//                 Bson::DateTime(date_time) => {
//                     info!(target:"db::execute::dtm","{}",date_time);
//                     db = db.bind(date_time.timestamp_millis());
//                 }
//                 Bson::Symbol(_) => {}
//                 Bson::Decimal128(decimal128) => {
//                     info!(target:"db::execute::dec","{}",decimal128);
//                     db = db.bind(decimal128.bytes());
//                 }
//                 _ => {}
//             };
//         }
//         let execute = db.fetch_optional(pool).await;
//         if execute.is_err() {
//             return Err(format!("{:?}", execute.err()));
//         }
//         let execute = execute.unwrap();
//         if execute.is_none() {
//             return Err("NOT RETURNING".to_string());
//         }
//         Ok(execute.unwrap())
//     }

//     pub fn select(mut self, select: &str) -> Self {
//         self.query += "SELECT ";
//         self.query += select;
//         self.query += " FROM ";
//         self.query += self.table_name.as_str();
//         self
//     }

//     pub fn join(mut self, table: &str, on: &str, alias: &str) -> Self {
//         self.query += " JOIN ";
//         self.query += table;
//         if !alias.is_empty() {
//             self.query += " ";
//             self.query += alias;
//         }
//         self.query += " ON ";

//         self.query += on;
//         self
//     }

//     pub fn inner_join(mut self, table: &str, on: &str, alias: &str) -> Self {
//         self.query += " INNER JOIN ";
//         self.query += table;
//         if !alias.is_empty() {
//             self.query += " ";
//             self.query += alias;
//         }
//         self.query += " ON ";

//         self.query += on;
//         self
//     }

//     pub fn outer_join(mut self, table: &str, on: &str, alias: &str) -> Self {
//         self.query += "OUTER JOIN ";
//         self.query += table;
//         if !alias.is_empty() {
//             self.query += " ";
//             self.query += alias;
//         }
//         self.query += " ON ";

//         self.query += on;
//         self
//     }

//     pub fn left_join(mut self, table: &str, on: &str, alias: &str) -> Self {
//         self.query += " LEFT JOIN ";
//         self.query += table;
//         if !alias.is_empty() {
//             self.query += " ";
//             self.query += alias;
//         }
//         self.query += " ON ";

//         self.query += on;
//         self
//     }

//     pub fn right_join(mut self, table: &str, on: &str, alias: &str) -> Self {
//         self.query += " RIGHT JOIN ";
//         self.query += table;
//         if !alias.is_empty() {
//             self.query += " ";
//             self.query += alias;
//         }
//         self.query += " ON ";

//         self.query += on;
//         self
//     }

//     pub fn full_join(mut self, table: &str, on: &str, alias: &str) -> Self {
//         self.query += " FULL JOIN ";
//         self.query += table;
//         if !alias.is_empty() {
//             self.query += " ";
//             self.query += alias;
//         }
//         self.query += " ON ";

//         self.query += on;
//         self
//     }

//     pub fn when(mut self, data: &[FilterGroup]) -> Self {
//         self.query += " WHERE ";
//         for (_, clause) in data.iter().enumerate() {
//             if self.args.len() > 0 {
//                 self.query += " AND ";
//             }
//             match clause {
//                 FilterGroup::BASIC(column) => {
//                     let mut q = column.0.clone();
//                     for (index, arg) in column.1.clone().iter().enumerate() {
//                         let to_replace = format!("${}", index);
//                         q = q.replace(&to_replace, format!("${}", self.args.len() + 1).as_str());
//                         self.args.push(arg.clone());
//                     }
//                     self.query += q.as_str();
//                 }
//                 FilterGroup::OR(items) => {
//                     self.query += "(";
//                     for (index, column) in items.iter().enumerate() {
//                         if index > 0 {
//                             self.query += " OR ";
//                         }
//                         let mut q = column.0.clone();
//                         for (index, arg) in column.1.clone().iter().enumerate() {
//                             let to_replace = format!("${}", index);
//                             q = q
//                                 .replace(&to_replace, format!("${}", self.args.len() + 1).as_str());
//                             self.args.push(arg.clone());
//                         }
//                         self.query += q.as_str();
//                     }
//                     self.query += ")";
//                 }
//                 FilterGroup::AND(items) => {
//                     self.query += "(";
//                     for (index, column) in items.iter().enumerate() {
//                         if index > 0 {
//                             self.query += " AND ";
//                         }
//                         let mut q = column.0.clone();
//                         for (index, arg) in column.1.clone().iter().enumerate() {
//                             let to_replace = format!("${}", index);
//                             q = q
//                                 .replace(&to_replace, format!("${}", self.args.len() + 1).as_str());
//                             self.args.push(arg.clone());
//                         }
//                         self.query += q.as_str();
//                     }
//                     self.query += ")";
//                 }
//             }
//         }
//         self.query += ";";
//         self
//     }

//     pub async fn fetch_all<T>(self, pool: &sqlx::Pool<Postgres>) -> Result<Vec<T>, String>
//     where
//         T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
//     {
//         info!(target:"db:query","{}",self.query.clone());
//         let mut db = sqlx::query_as::<_, T>(&self.query);
//         for bind in self.args {
//             match bind {
//                 Bson::Double(value) => {
//                     db = db.bind(value);
//                 }
//                 Bson::String(value) => {
//                     db = db.bind(value);
//                 }
//                 Bson::Array(bsons) => {
//                     let data = bsons
//                         .iter()
//                         .map(|v| v.to_json_string().unwrap_or(String::new()))
//                         .collect::<Vec<String>>();
//                     db = db.bind(data);
//                 }
//                 Bson::Document(_) => {}
//                 Bson::Boolean(value) => {
//                     db = db.bind(value);
//                 }
//                 Bson::Null => {
//                     db = db.bind("NULL");
//                 }
//                 Bson::RegularExpression(regex) => {
//                     db = db.bind(regex.pattern);
//                 }

//                 Bson::Int32(value) => {
//                     db = db.bind(value);
//                 }
//                 Bson::Int64(value) => {
//                     db = db.bind(value);
//                 }
//                 Bson::Timestamp(timestamp) => {
//                     db = db.bind(timestamp.time as i64);
//                 }
//                 Bson::Binary(_) => {}
//                 Bson::ObjectId(object_id) => {
//                     db = db.bind(object_id.to_string());
//                 }
//                 Bson::DateTime(date_time) => {
//                     db = db.bind(date_time.timestamp_millis());
//                 }
//                 Bson::Symbol(_) => {}
//                 Bson::Decimal128(decimal128) => {
//                     db = db.bind(decimal128.bytes());
//                 }
//                 _ => {}
//             };
//         }
//         let data = db.fetch_all(pool).await;
//         if data.is_err() {
//             let message = format!("{:?}", data.err());
//             info!(target:"fetch::all::error","{}", message.clone());
//             return Err(message);
//         }
//         Ok(data.unwrap())
//     }

//     pub fn build_query(self) -> String {
//         self.query
//     }
// }

// pub enum FilterGroup {
//     BASIC((String, Vec<Bson>)),
//     OR(Vec<(String, Vec<Bson>)>),
//     AND(Vec<(String, Vec<Bson>)>),
// }

// pub fn and(value: &[FilterGroup]) -> FilterGroup {
//     let mut v = Vec::new();
//     for group in value {
//         match group {
//             FilterGroup::BASIC(b) => {
//                 v.push((b.0.clone(), b.1.clone()));
//             }
//             _ => {}
//         }
//     }

//     FilterGroup::AND(v)
// }
// pub fn or(value: &[FilterGroup]) -> FilterGroup {
//     let mut v = Vec::new();
//     for group in value {
//         match group {
//             FilterGroup::BASIC(b) => {
//                 v.push((b.0.clone(), b.1.clone()));
//             }
//             _ => {}
//         }
//     }

//     FilterGroup::OR(v)
// }

// pub fn equal<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
//     let q = format!("{}=$0", column);
//     let bsons: Vec<Bson> = vec![value.into()];
//     FilterGroup::BASIC((q, bsons))
// }

// pub fn like<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
//     let q = format!("{} LIKE $0", column);
//     let bsons: Vec<Bson> = vec![value.into()];
//     FilterGroup::BASIC((q, bsons))
// }

// pub fn ilike<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
//     let q = format!("{} ILIKE $0", column);
//     let bsons: Vec<Bson> = vec![value.into()];
//     FilterGroup::BASIC((q, bsons))
// }

// pub fn gt<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
//     let q = format!("{} > $0", column);
//     let bsons: Vec<Bson> = vec![value.into()];
//     FilterGroup::BASIC((q, bsons))
// }

// pub fn gte<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
//     let q = format!("{} >= $0", column);
//     let bsons: Vec<Bson> = vec![value.into()];
//     FilterGroup::BASIC((q, bsons))
// }

// pub fn lt<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
//     let q = format!("{} < $0", column);
//     let bsons: Vec<Bson> = vec![value.into()];
//     FilterGroup::BASIC((q, bsons))
// }

// pub fn lte<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
//     let q = format!("{} <= $0", column);
//     let bsons: Vec<Bson> = vec![value.into()];
//     FilterGroup::BASIC((q, bsons))
// }

// #[cfg(test)]
// mod test {

//     use crate::common::orm::orm::DB;

//     fn test() {}
// }
