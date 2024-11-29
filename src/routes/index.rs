use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::lang::Lang;
use crate::common::minio::MinIO;
use crate::common::orm::orm::Orm;
use crate::dto::account_permission_dto::AccountPermissionDTO;
use crate::entity::account_permission::AccountPermission;
use crate::translate;
use axum::extract::State;
use bson::oid::ObjectId;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

pub async fn index(
    state: State<AppState>
) -> ApiResponse<Vec<AccountPermissionDTO>> {
    let command = Orm::get("account-permission")
        .filter_object_id("account_id", &ObjectId::from_str("6742c74a15e68b0e7ee06122").unwrap_or(ObjectId::new()));

    let find = command
        .all::<AccountPermissionDTO>(&state.db).await;

    let update = Orm::get("account")
        .and()
        .filter_object_id("_id", &ObjectId::new())
        .filter_object_id("_id2", &ObjectId::new());
    ApiResponse::ok(find.unwrap_or(Vec::new()), "test merge")
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

pub async fn test_locales(
    lang: Lang,
) -> ApiResponse<HashMap<String, String>> {
    let re1 = translate!("message",lang.get(),{"name"=>"trian","hohoe"=>"Tes"});
    let re2 = translate!("message",lang.get(),{"name"=>"trian","hohoe"=>"Tes"});
    let re3 = translate!("message", lang.get());

    let mut hashmap = HashMap::new();
    hashmap.insert("message1".to_string(), re1);
    hashmap.insert("message2".to_string(), re2);
    hashmap.insert("message3".to_string(), re3);
    ApiResponse::ok(hashmap, "")
}
