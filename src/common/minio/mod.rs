use crate::common::env_config::EnvConfig;
use log::info;
use s3::creds::Credentials;
use s3::request::ResponseData;
use s3::{Bucket, Region};

pub struct MinIO {
    access_key: String,
    secret_key: String,
    url_server: String,
}

impl MinIO {
    pub async fn new() -> MinIO {
        let env = EnvConfig::init();
        MinIO {
            access_key: env.minio_access_key.clone(),
            secret_key: env.minio_secret_key.clone(),
            url_server: env.minio_url.clone(),
        }
    }

    pub async fn get_file(
        &self,
        bucket_name: String,
        file_name: String,
    ) -> Result<ResponseData, String> {
        let credentials = Credentials::new(Some(self.access_key.clone().as_str()), Some(self.secret_key.clone().as_str()), None, None, None);
        if credentials.is_err() {
            return Err(credentials.unwrap_err().to_string());
        }
        let credentials = credentials.unwrap();
        let bucket = Bucket::new(
            bucket_name.as_str(),
            Region::Custom {
                region: "asia".to_string(),
                endpoint: self.url_server.clone(),
            },
            credentials,
        );
        if bucket.is_err() {
            return Err(bucket.unwrap_err().to_string());
        }
        let filename = format!("/{}", file_name);
        let bucket = bucket.unwrap().with_path_style();

        let file = bucket.get_object(filename.clone()).await;
        if file.is_err() {
            return Err(file.unwrap_err().to_string());
        }

        {
            info!(target: "get_object","from minio {}", &filename);
        }
        if file.is_err() {
            return Err("".to_string());
        }
        let file = file.unwrap();
        Ok(file)
    }

    pub async fn upload_file(&self, file_path: String, bucket_name: String, file_name: String) -> Result<String, String> {
        let credentials = Credentials::new(Some(self.access_key.clone().as_str()), Some(self.secret_key.clone().as_str()), None, None, None);
        if credentials.is_err() {
            return Err(credentials.unwrap_err().to_string());
        }
        let credentials = credentials.unwrap();
        let bucket = Bucket::new(
            bucket_name.as_str(),
            Region::Custom {
                region: "asia".to_string(),
                endpoint: self.url_server.clone(),
            },
            credentials,
        );
        if bucket.is_err() {
            return Err(bucket.unwrap_err().to_string());
        }
        let bucket = bucket.unwrap().with_path_style();


        let file = tokio::fs::read(file_path).await;
        if file.is_err() {
            return Err(file.unwrap_err().to_string());
        }
        let file = file.unwrap();
        let upload = bucket
            .put_object(file_name.as_str(), &file)
            .await;
        match upload {
            Ok(_) => Ok("Successfully uploaded file".to_string()),
            Err(e) => {
                Err(format!("Error uploading file: {}", e))
            }
        }
    }

    pub async fn delete_file(&self, file_path: String, bucket_name: String) -> Result<String, String> {
        let credentials = Credentials::new(Some(self.access_key.clone().as_str()), Some(self.secret_key.clone().as_str()), None, None, None);
        if credentials.is_err() {
            return Err(credentials.unwrap_err().to_string());
        }
        let credentials = credentials.unwrap();
        let bucket = Bucket::new(
            bucket_name.as_str(),
            Region::Custom {
                region: "asia".to_string(),
                endpoint: self.url_server.clone(),
            },
            credentials,
        );
        if bucket.is_err() {
            return Err(bucket.unwrap_err().to_string());
        }
        let bucket = bucket.unwrap().with_path_style();

        let upload = bucket
            .delete_object(file_path.as_str())
            .await;
        match upload {
            Ok(_) => Ok("Successfully DELETE file".to_string()),
            Err(e) => {
                Err(format!("Error uploading file: {}", e))
            }
        }
    }
}