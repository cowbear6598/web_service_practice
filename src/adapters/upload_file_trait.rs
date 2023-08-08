pub trait UploadFileTrait: Send + Sync {
    fn get_filename(&self) -> String;
    fn get_file(&self) -> Vec<u8>;
    fn get_file_type(&self) -> String;
}