use mime::Mime;

pub struct UploadFile {
    pub filename: String,
    pub content: Vec<u8>,
    pub file_type: Mime,
}