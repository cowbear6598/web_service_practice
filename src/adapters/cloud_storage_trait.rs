use async_trait::async_trait;
use shaku::Interface;

#[async_trait]
pub trait CloudStorageTrait: Send + Sync + Interface {}