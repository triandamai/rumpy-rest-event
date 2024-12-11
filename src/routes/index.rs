use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::lang::Lang;
use crate::common::minio::MinIO;
use crate::common::orm::orm::Orm;
use crate::translate;
use axum::extract::State;
use bson::Document;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub async fn index(_state: State<AppState>) -> ApiResponse<(Vec<Document>, Vec<Document>)> {
    let _command = Orm::get("permission");

    let find_all_branch = Orm::get("account")
        .group_by_desc("branch_name")
        .join_one("account", "branch_owner", "_id", "owner")
        .filter_bool("deleted", None, false);
    ApiResponse::ok(find_all_branch.show_merging(), "test merge")
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
