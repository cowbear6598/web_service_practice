use std::env;
use anyhow::Result;
use async_trait::async_trait;
use google_cloud_storage::{
    client::{Client, ClientConfig},
    http::objects::upload::{UploadObjectRequest, UploadType},
    http::objects::Object,
};
use shaku::Component;
use crate::{
    adapters::cloud_storage_trait::CloudStorageTrait,
    adapters::upload_file_trait::UploadFileTrait,
};

#[derive(Component)]
#[shaku(interface = CloudStorageTrait)]
pub struct CloudStorage {
    client: Client,
    bucket: String,
    base_url: String,
}

#[async_trait]
impl CloudStorageTrait for CloudStorage {
    async fn upload(&self, file: &dyn UploadFileTrait) -> Result<String> {
        let metadata = Object {
            name: file.get_filename(),
            content_type: Some(file.get_file_type()),
            ..Default::default()
        };

        let upload_type = UploadType::Multipart(Box::new(metadata));

        let upload_req = UploadObjectRequest {
            bucket: self.bucket.clone(),
            ..Default::default()
        };

        let object = self.client.upload_object(
            &upload_req,
            file.get_file(),
            &upload_type,
        ).await?;

        Ok(format!("{}{}", self.base_url, object.name))
    }
}

pub async fn cloud_storage_connect() -> CloudStorageParameters {
    let bucket = env::var("GCS_BUCKET").
        expect("請設定 Bucket 環境變數");

    let config = ClientConfig::default().with_auth().await
        .expect("請設定 GCS 權限變數");

    let client = Client::new(config);

    let base_url = format!("https://storage.googleapis.com/{}/", bucket);

    CloudStorageParameters {
        client,
        bucket,
        base_url,
    }
}