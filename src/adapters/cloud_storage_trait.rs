use async_trait::async_trait;
use shaku::Interface;
use anyhow::Result;
use crate::adapters::upload_file_trait::UploadFileTrait;

#[async_trait]
pub trait CloudStorageTrait: Send + Sync + Interface {
    async fn upload(&self, file: &dyn UploadFileTrait) -> Result<String>;
}