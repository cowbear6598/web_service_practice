use mime::{APPLICATION_OCTET_STREAM, IMAGE_JPEG, IMAGE_PNG, Mime};
use anyhow::{anyhow, Result};
use crate::{
    adapters::upload_file_trait::UploadFileTrait,
    frameworks::errors::file_error::FileError
};

#[derive(Clone)]
pub struct ImageFile {
    max_size: usize,
    mime_types: Vec<Mime>,

    upload_dir: String,
    file: Vec<u8>,
    file_type: Mime,
    file_size: usize,
    filename: String,
}

impl UploadFileTrait for ImageFile {
    fn get_filename(&self) -> String {
        format!("{}{}", self.filename, self.get_extension())
    }

    fn get_file(&self) -> Vec<u8> {
        self.file.clone()
    }

    fn get_upload_dir(&self) -> String {
        self.upload_dir.clone()
    }

    fn validate(&self) -> Result<()> {
        if self.file_size > self.max_size || self.file_size == 0 {
            return Err(anyhow!(FileError::SizeError));
        }

        if !self.mime_types.contains(&self.file_type) {
            return Err(anyhow!(FileError::TypeNotAllow));
        }

        if self.filename == "" {
            return Err(anyhow!(FileError::FileNameEmpty));
        }

        if self.upload_dir == "" {
            return Err(anyhow!(FileError::UploadDirEmpty));
        }

        if self.file.len() == 0 {
            return Err(anyhow!(FileError::FileContentEmpty));
        }

        Ok(())
    }
}

impl ImageFile {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_upload_dir(mut self, upload_dir: String) -> Self {
        self.upload_dir = upload_dir;
        self
    }

    pub fn set_file_type(mut self, file_type: Mime) -> Self {
        self.file_type = file_type;
        self
    }

    pub fn set_file_size(mut self, file_size: usize) -> Self {
        self.file_size = file_size;
        self
    }

    pub fn set_file(mut self, file: Vec<u8>) -> Self {
        self.file = file;
        self
    }

    pub fn set_filename(mut self, filename: String) -> Self {
        self.filename = filename;
        self
    }

    fn get_extension(&self) -> &str {
        match self.file_type.to_string().as_str() {
            "image/jpeg" => ".jpg",
            "image/png" => ".png",
            _ => "",
        }
    }
}

impl Default for ImageFile {
    fn default() -> Self {
        ImageFile {
            max_size: 1024 * 1024 * 5,
            mime_types: vec![IMAGE_JPEG, IMAGE_PNG],

            upload_dir: "".to_string(),
            file_type: APPLICATION_OCTET_STREAM,
            file_size: 0,
            file: vec![],
            filename: "".to_string(),
        }
    }
}