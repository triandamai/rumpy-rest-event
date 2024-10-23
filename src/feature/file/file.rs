use crate::common::api_response::ApiResponse;
use crate::common::app_state::AppState;
use crate::common::env_config::EnvConfig;
use crate::common::jwt::JwtClaims;
use crate::common::minio::MinIO;
use crate::entity::thread_attachment::{ThreadAttachment, ThreadAttachmentDTO};
use crate::feature::file::file_model::UploadFileAttachment;
use axum::extract::{Multipart, Path, State};
use chrono::Utc;
use log::info;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, DateTime};
use std::fs;
use std::str::FromStr;
use validator::Validate;

pub async fn upload_thread_attachment(
    state: State<AppState>,
    _auth: JwtClaims,
    multipart: Multipart,
) -> ApiResponse<ThreadAttachmentDTO> {
    let metadata = UploadFileAttachment::extract_multipart(multipart).await;
    let validate = metadata.validate();
    if validate.is_err() {
        return ApiResponse::failed(validate.unwrap_err().to_string());
    }

    info!(target: "upload_attachment","{:?}",metadata);

    let thread_id = ObjectId::from_str(metadata.ref_id.as_str());
    if thread_id.is_err() {
        let _ = metadata.remove_file();
        return ApiResponse::failed(thread_id.unwrap_err().to_string());
    }
    let current_time = DateTime::now();
    let thread_id = thread_id.unwrap();
    let filename = format!("{}-{}.{}", metadata.ref_id.clone(), current_time.timestamp_millis(), metadata.extension);
    let mut thread_attachment = ThreadAttachment {
        id: None,
        ref_id: Some(thread_id),
        filename: filename.clone(),
        mime_type: metadata.mime_type.clone(),
        created_at: current_time.clone(),
        updated_at: current_time.clone(),
    };

    let minio = MinIO::new().await;
    let put_obj = minio
        .upload_file(metadata.temp_path.clone(), "threads".to_string(), filename)
        .await;
    if put_obj.is_err() {
        let _ = metadata.remove_file();
        return ApiResponse::failed(put_obj.unwrap_err().to_string());
    }

    let upload = thread_attachment.save(
        &state.db
    ).await;
    if upload.is_none() {
        let _ = metadata.remove_file();
        return ApiResponse::failed(put_obj.unwrap_err().to_string());
    }

    let _remove = metadata.remove_file();
    ApiResponse::ok(upload.unwrap().to_dto(), "Berhasil mengupload")
}

pub async fn update_thread_attachment(
    mut state: State<AppState>,
    _auth: JwtClaims,
    multipart: Multipart,
) -> ApiResponse<ThreadAttachmentDTO> {
    let metadata = UploadFileAttachment::extract_multipart(multipart).await;
    let validate = metadata.validate();
    if validate.is_err() {
        return ApiResponse::failed(validate.unwrap_err().to_string());
    }

    info!(target: "update_attachment","{:?}",metadata);

    let thread_id = ObjectId::from_str(metadata.ref_id.as_str());
    if thread_id.is_err() {
        let _ = metadata.remove_file();
        return ApiResponse::failed(thread_id.unwrap_err().to_string());
    }
    let attachment_id = thread_id.unwrap();

    let find_attachment = ThreadAttachment::find_one(doc! {
        "_id": attachment_id.clone(),
    }, &state.db).await;

    if find_attachment.is_none() {
        let _ = metadata.remove_file();
        return ApiResponse::failed("Attachment tidak ditemukan".to_string());
    }
    let mut find_attachment = find_attachment.unwrap();

    let current_time = DateTime::now();
    let minio = MinIO::new().await;
    let ref_id = match find_attachment.ref_id.clone() {
        Some(id) => Some(id.to_string()),
        None => None
    };
    if ref_id.is_none() {
        return ApiResponse::failed("Attachment tidak terhubung ke postingan manapun.".to_string());
    }

    let filename = format!("{}-{}.{}", ref_id.unwrap(), current_time.timestamp_millis(), metadata.extension);

    let put_obj = minio
        .upload_file(metadata.temp_path.clone(), "threads".to_string(), filename.clone())
        .await;
    if put_obj.is_err() {
        let _ = metadata.remove_file();
        return ApiResponse::failed(put_obj.unwrap_err().to_string());
    }

    let _remove_obj = minio
        .delete_file(find_attachment.filename, "threads".to_string())
        .await;

    find_attachment.updated_at = current_time.clone();
    find_attachment.mime_type = metadata.mime_type.clone();
    find_attachment.filename = filename.clone();


    let time = bson::DateTime::now().try_to_rfc3339_string();
    let upload = ThreadAttachment::update_one(
        doc! {"_id":attachment_id.clone()},
        doc! {"$set":{
            "filename":filename.clone(),
            "mime_type":metadata.mime_type.clone(),
            "updated_at":time.unwrap()
        }},
        &state.db,
    ).await;
    if upload.is_err() {
        let _ = metadata.remove_file();
        return ApiResponse::failed(put_obj.unwrap_err().to_string());
    }

    let _remove = metadata.remove_file();
    ApiResponse::ok(find_attachment.to_dto(), "Berhasil mengupload")
}

pub async fn delete_thread_attachment(
    state: State<AppState>,
    _auth: JwtClaims,
    Path(attachment_id): Path<String>,
) -> ApiResponse<ThreadAttachmentDTO> {
    let attachment_id = ObjectId::from_str(attachment_id.as_str());
    if attachment_id.is_err() {
        return ApiResponse::failed(attachment_id.unwrap_err().to_string());
    }
    let attachment_id = attachment_id.unwrap();

    let find = ThreadAttachment::find_one(doc! {
            "_id": attachment_id.clone()
    }, &state.db)
        .await;

    if find.is_none() {
        return ApiResponse::failed("Tidak dapat menemukan data".to_string());
    }
    let mut find = find.unwrap();

    let minio = MinIO::new().await;
    let remove = minio
        .delete_file(find.filename.clone(), "threads".to_string())
        .await;
    if remove.is_err() {
        return ApiResponse::failed(remove.unwrap_err().to_string());
    }

    let delete_from_db = find
        .delete(doc! {"_id": attachment_id.clone()}, &state.db)
        .await;

    if delete_from_db.is_err() {
        return ApiResponse::failed(delete_from_db.unwrap_err().to_string());
    }
    let delete = delete_from_db.unwrap();
    ApiResponse::ok(delete.to_dto(), "Delete success")
}