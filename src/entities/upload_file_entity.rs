use mime::{IMAGE_JPEG, IMAGE_PNG, Mime};

pub struct UploadFile {
    max_size: usize,
    file_size: usize,
    mime_types: Vec<Mime>,
    upload_dir: String,
    pub filename: String,
}

impl UploadFile {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn max_size(mut self, max_size: usize) -> Self {
        self.max_size = max_size;
        self
    }

    pub fn file_size(mut self, file_size: usize) -> Self {
        self.file_size = file_size;
        self
    }

    pub fn mime_types(mut self, mime_types: Vec<Mime>) -> Self {
        self.mime_types = mime_types;
        self
    }

    pub fn upload_dir(mut self, upload_dir: String) -> Self {
        self.upload_dir = upload_dir;
        self
    }

    pub fn filename(mut self, filename: String) -> Self {
        self.filename = filename;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

impl Default for UploadFile {
    fn default() -> Self {
        UploadFile {
            max_size: 1024 * 1024 * 1, // 5MB
            file_size: 0,
            mime_types: vec![IMAGE_JPEG, IMAGE_PNG],
            upload_dir: "".to_string(),
            filename: "unknown".to_string(),
        }
    }
}