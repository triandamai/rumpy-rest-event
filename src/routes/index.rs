use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::lang::Lang;
use crate::common::minio::MinIO;
use crate::translate;
use axum::extract::State;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use bson::Document;
use bson::oid::ObjectId;
use crate::common::orm::orm::Orm;

pub async fn index(_state: State<AppState>) -> ApiResponse<Vec<Document>> {
    let find_all_branch = Orm::get("product")
        .and()
        .filter_bool("deleted", None, false)
        .filter_object_id("branch_id", &ObjectId::new())
        .join_one("file-attachment", "_id", "ref_id", "product_image")
        .join_one("account", "created_by_id", "_id", "created_by").show_merging();

    ApiResponse::ok(find_all_branch.1, "test merge")
}

pub async fn generate_locales() -> ApiResponse<String> {
    let minio = MinIO::new()
        .await
        .get_file("assets".to_string(), "locales.json".to_string())
        .await;
    if minio.is_err() {
        return ApiResponse::failed(minio.unwrap_err().as_str());
    }
    let file = minio.unwrap();

    // Read the file contents
    let data = file.bytes();

    // Save the file to the "uploads" folder
    let mut file = File::create(format!("locales/{}", "app.json")).unwrap();
    file.write_all(&data).unwrap();
    ApiResponse::ok(String::from("OK"), "success generate locales")
}

pub async fn test_locales(lang: Lang) -> ApiResponse<HashMap<String, String>> {
    let re1 = translate!("message",lang,{"name"=>"trian","hohoe"=>"Tes"});
    let re2 = translate!("message",lang,{"name"=>"trian","hohoe"=>"Tes"});
    let re3 = translate!("message", lang);

    let mut hashmap: HashMap<String, String> = HashMap::new();
    hashmap.insert("message1".to_string(), re1.to_string());
    hashmap.insert("message2".to_string(), re2.to_string());
    hashmap.insert("message3".to_string(), re3.to_string());
    ApiResponse::ok(hashmap, "")
}
