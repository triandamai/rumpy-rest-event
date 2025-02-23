use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::lang::{self, Lang};
use crate::common::minio::MinIO;

use crate::entity::user::User;
use crate::schema::tb_user;
use crate::{i18n, schema, t};
use axum::extract::State;
use diesel::prelude::Insertable;
use diesel::{RunQueryDsl, SelectableHelper};
use std::fs::File;
use std::io::Write;

#[derive(Insertable)]
#[diesel(table_name=tb_user)]
pub struct CreateUser<'a> {
    pub display_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

pub async fn index(state: State<AppState>, lang: Lang) -> ApiResponse<User> {
    let mut conn = state.postgres.get();
    if conn.is_err() {
        return ApiResponse::failed(format!("{:?}", conn.err()).as_str());
    }

    let result: Result<User, diesel::result::Error> = diesel::insert_into(tb_user::table)
        .values(&CreateUser {
            display_name: "sasa",
            email: "test@gmail.com",
            password: "sasa",
        })
        .returning(User::as_returning())
        .get_result(&mut conn.unwrap());
    return ApiResponse::ok(result.unwrap(), "");
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
