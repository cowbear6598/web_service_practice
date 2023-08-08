use std::env;
use anyhow::Result;
use async_trait::async_trait;
use google_cloud_storage::{
    client::{Client, ClientConfig},
    http::objects::upload::{Media, UploadObjectRequest, UploadType},
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
    bucket_name: String,
}

#[async_trait]
impl CloudStorageTrait for CloudStorage {
    async fn upload(&self, file: &dyn UploadFileTrait) -> Result<String> {
        file.validate()?;

        let upload_type = UploadType::Simple(
            Media::new(file.get_filename())
        );

        // let bucket = format!("{}/{}", self.bucket_name, file.get_upload_dir());
        let bucket = self.bucket_name.clone();

        let upload_req = UploadObjectRequest {
            bucket,
            ..Default::default()
        };

        let object = self.client.upload_object(
            &upload_req,
            file.get_file(),
            &upload_type,
        ).await?;

        Ok(object.self_link)
    }
}

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