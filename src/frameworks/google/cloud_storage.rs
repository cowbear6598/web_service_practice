use std::env;
use std::io::Write;
use actix_multipart::Field;
use anyhow::Result;
use async_trait::async_trait;
use futures_util::TryStreamExt;
use google_cloud_storage::{
    client::{Client, ClientConfig},
    http::objects::upload::{Media, UploadObjectRequest, UploadType},
};
use shaku::Component;
use crate::{
    adapters::cloud_storage_trait::CloudStorageTrait,
    entities::upload_file_entity::UploadFile,
};

#[derive(Component)]
#[shaku(interface = CloudStorageTrait)]
pub struct CloudStorage {
    client: Client,
    bucket_name: String,
}

impl CloudStorage {
    pub async fn upload_image(&self, mut field: Field, file: UploadFile) -> Result<String> {
        let mut buffer = vec![];

        while let Ok(Some(chunk)) = field.try_next().await {
            buffer.write_all(&chunk)?
        }

        let upload_type = UploadType::Simple(Media::new(file.filename));

        let object = self.client.upload_object(&UploadObjectRequest {
            bucket: self.bucket_name.clone(),
            ..Default::default()
        }, buffer, &upload_type).await?;

        Ok(object.self_link)
    }
}

#[async_trait]
impl CloudStorageTrait for CloudStorage {}

pub async fn cloud_storage_connect() -> CloudStorageParameters {
    let bucket_name = env::var("GCS_BUCKET").
        expect("請設定 Bucket 環境變數");

    let config = ClientConfig::default().with_auth().await
        .expect("請設定 GCS 權限變數");

    let client = Client::new(config);

    CloudStorageParameters {
        client,
        bucket_name,
    }
}