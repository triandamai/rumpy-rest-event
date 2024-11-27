use crate::common::api_response::ApiResponse;
use crate::common::lang::Lang;
use crate::common::minio::MinIO;
use crate::common::orm::orm::Orm;
use crate::translate;
use bson::oid::ObjectId;
use bson::Document;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub async fn index() -> ApiResponse<Vec<Document>> {
    let update = Orm::get("account")
        .and()
        .filter_object_id("_id", &ObjectId::new())
        .filter_object_id("_id2", &ObjectId::new());
    ApiResponse::ok(update.show_merging(), "test merge")
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
