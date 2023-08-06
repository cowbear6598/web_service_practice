use std::env;
use std::io::Write;
use actix_multipart::Field;
use anyhow::Result;
use futures_util::TryStreamExt;
use google_cloud_storage::client::{Client, ClientConfig};

pub async fn upload_image(mut field: Field) -> Result<String> {
    let mut buffer = vec![];

    while let Ok(Some(chunk)) = field.try_next().await {
        buffer.write_all(&chunk)?
    }

    let service_path = env::var("GCS_KEY")?;
    let bucket_name = env::var("GCS_BUCKET")?;


    let cliet = Client::default();

    let client = Client::default();

    let result = client.object().create(bucket_name.as_str(), buffer, "test.png", "image/png").await?;

    Ok(result.media_link)
}