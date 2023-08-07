use mime::{IMAGE_JPEG, IMAGE_PNG, Mime};
use crate::adapters::upload_file_trait::UploadFileTrait;

pub struct ImageFile {
    max_size: usize,
    mime_types: Vec<Mime>,

    upload_dir: Option<String>,
    file: Option<Vec<u8>>,
    file_type: Option<Mime>,
    file_size: Option<usize>,
    filename: Option<String>,
}

impl UploadFileTrait for ImageFile {
    fn get_filename(&self) -> String {
        format!("{} {}", self.filename.unwrap(), self.get_extension())
    }

    fn get_file(&self) -> Vec<u8> {
        self.file.unwrap()
    }

    fn get_upload_dir(&self) -> String {
        self.upload_dir.unwrap()
    }
}

impl ImageFile {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_upload_dir(mut self, upload_dir: String) -> Self {
        self.upload_dir = Some(upload_dir);
        self
    }

    pub fn set_file_type(mut self, file_type: Mime) -> Self {
        self.file_type = Some(file_type);
        self
    }

    pub fn set_file_size(mut self, file_size: usize) -> Self {
        self.file_size = Some(file_size);
        self
    }

    pub fn set_file(mut self, file: Vec<u8>) -> Self {
        self.file = Some(file);
        self
    }

    pub fn set_filename(mut self, filename: String) -> Self {
        self.filename = Some(filename);
        self
    }

    fn get_extension(&self) -> &str {
        if self.file_type.is_none() {
            return "";
        }

        match self.file_type {
            Some(IMAGE_JPEG) => ".jpg",
            Some(IMAGE_PNG) => ".png",
            None => "",
        }
    }
}

impl Default for ImageFile {
    fn default() -> Self {
        ImageFile {
            max_size: 1024 * 1024 * 5,
            mime_types: vec![IMAGE_JPEG, IMAGE_PNG],

            upload_dir: None,
            file_type: None,
            file_size: None,
            file: None,
            filename: None,
        }
    }
}