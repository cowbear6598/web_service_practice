use anyhow::Result;

pub trait UploadFileTrait: Send + Sync {
    fn get_filename(&self) -> String;
    fn get_file(&self) -> Vec<u8>;
    fn get_upload_dir(&self) -> String;
    fn validate(&self) -> Result<()>;
}