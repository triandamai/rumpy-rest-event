use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::lang::Lang;
use crate::common::minio::MinIO;
use crate::common::orm::orm::Orm;
use crate::entity::permission::Permission;
use crate::translate;
use axum::extract::State;
use bson::oid::ObjectId;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use crate::dto::permission_dto::PermissionDTO;

pub async fn index(state: State<AppState>) -> ApiResponse<Vec<PermissionDTO>> {
    let command = Orm::get("permission");

     let find = command.clone().all::<PermissionDTO>(&state.db).await;

    let mut get = Orm::get("branch")
        .filter_object_id("_id", &ObjectId::new())
        .filter_string("tes", None, "hahah")
        .group_by_asc("branch_name");

    ApiResponse::ok(find.unwrap(), "test merge")
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
    let re1 = translate!("message",lang.get(),{"name"=>"trian","hohoe"=>"Tes"});
    let re2 = translate!("message",lang.get(),{"name"=>"trian","hohoe"=>"Tes"});
    let re3 = translate!("message", lang.get());

    let mut hashmap = HashMap::new();
    hashmap.insert("message1".to_string(), re1);
    hashmap.insert("message2".to_string(), re2);
    hashmap.insert("message3".to_string(), re3);
    ApiResponse::ok(hashmap, "")
}
