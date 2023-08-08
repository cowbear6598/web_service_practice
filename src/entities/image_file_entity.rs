use mime::{APPLICATION_OCTET_STREAM, IMAGE_JPEG, IMAGE_PNG, Mime};
use anyhow::{anyhow, Result};
use crate::{
    adapters::upload_file_trait::UploadFileTrait,
    frameworks::errors::file_error::FileError,
};

#[derive(Clone)]
pub struct ImageFile {
    max_size: usize,
    mime_types: Vec<Mime>,

    upload_dir: String,
    file: Vec<u8>,
    file_type: Mime,
    filename: String,
}

impl UploadFileTrait for ImageFile {
    fn get_filename(&self) -> String {
        format!("{}/{}{}", self.upload_dir, self.filename, self.get_extension())
    }

    fn get_file(&self) -> Vec<u8> {
        self.file.clone()
    }

    fn get_file_type(&self) -> String {
        self.file_type.to_string()
    }
}

impl ImageFile {
    pub fn new(
        filename: String,
        upload_dir: String,
        file_type: Mime,
        file: Vec<u8>,
    ) -> Result<Self> {
        let image_file = Self {
            filename,
            upload_dir,
            file_type,
            file,

            ..Default::default()
        };

        image_file.validate()?;

        Ok(image_file)
    }

    fn validate(&self) -> Result<()> {
        if self.file.len() > self.max_size {
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
            file: vec![],
            filename: "".to_string(),
        }
    }
}