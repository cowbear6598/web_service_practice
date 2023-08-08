use std::collections::HashMap;
use std::io::Write;
use actix_multipart::Field;
use anyhow::{anyhow, Result};
use futures_util::{TryStreamExt};
use mongodb::bson::Uuid;
use crate::{
    adapters::cloud_storage_trait::CloudStorageTrait,
    entities::image_file_entity::ImageFile,
    frameworks::errors::multipart_error::MultipartError,
};

const MAX_TEXT_SIZE: usize = 1 * 1024;

pub enum Method {
    Text,
    Image,
}

struct FieldInfo {
    method: Method,
    filename: Option<String>,
    upload_dir: Option<String>,
}

pub struct MultipartHandler<'a> {
    cloud_storage: &'a dyn CloudStorageTrait,
    fields: HashMap<String, FieldInfo>,
}

impl<'a> MultipartHandler<'a> {
    pub fn new(cloud_storage: &'a dyn CloudStorageTrait) -> Self {
        Self {
            cloud_storage,
            fields: HashMap::new(),
        }
    }

    pub fn add_field(mut self, field_name: &str, method: Method, filename: Option<String>, upload_dir: Option<String>) -> Self {
        self.fields.insert(field_name.to_string(), FieldInfo {
            method,
            filename,
            upload_dir,
        });
        self
    }

    pub fn has_same_field(&self, map: &HashMap<String, String>) -> bool {
        if self.fields.len() != map.len() {
            return false;
        }

        for key in map.keys() {
            if !self.fields.contains_key(key) {
                return false;
            }
        }

        true
    }

    pub async fn handle(&self, field_name: &str, mut field: Field) -> Result<String> {
        let field_info = self.fields.get(field_name)
            .ok_or(anyhow!(MultipartError::FieldNameNotValid))?;

        let result = match field_info.method {
            Method::Text => self.process_text(&mut field).await?,
            Method::Image => self.process_image(&field_info, &mut field).await?,
        };

        Ok(result)
    }

    async fn process_text(&self, field: &mut Field) -> Result<String> {
        let buffer = self.get_field_buffer(field, 512).await?;

        if buffer.len() > MAX_TEXT_SIZE {
            return Err(anyhow!(MultipartError::TextSizeTooBig));
        }

        let text = String::from_utf8(buffer)?;

        Ok(text)
    }

    async fn process_image(&self, field_info: &FieldInfo, field: &mut Field) -> Result<String> {
        let file_type = field.content_type()
            .ok_or(anyhow!(MultipartError::ContentTypeGetFailed))?
            .clone();

        let filename = field_info.filename.clone()
            .unwrap_or(Uuid::new().to_string());
        let upload_dir = field_info.upload_dir.clone()
            .ok_or(anyhow!(MultipartError::DataNotFound))?;

        let buffer = self.get_field_buffer(field, 1024 * 500).await?;

        let image_file = ImageFile::new(
            filename,
            upload_dir,
            file_type,
            buffer,
        )?;

        let img_url = self.cloud_storage.upload(&image_file).await?;

        Ok(img_url)
    }

    async fn get_field_buffer(&self, field: &mut Field, capacity: usize) -> Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(capacity);

        while let Ok(Some(chunk)) = field.try_next().await {
            buffer.write_all(&chunk)?;
        }

        Ok(buffer)
    }
}

pub fn get_field_name(field: &Field) -> Result<String> {
    let content_disposition = field.content_disposition().clone();
    let field_name = content_disposition.get_name()
        .ok_or(anyhow!(MultipartError::FieldNameNotValid))?;

    Ok(field_name.to_string())
}