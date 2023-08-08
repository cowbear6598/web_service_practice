use std::io::Write;
use actix_multipart::Field;
use actix_web::http::header::CONTENT_LENGTH;
use actix_web::HttpRequest;
use anyhow::{anyhow, Result};
use futures_util::TryStreamExt;
use crate::{
    entities::upload_file_entity::ImageFile,
    frameworks::errors::multipart_error::MultipartError,
};
use crate::adapters::upload_file_trait::UploadFileTrait;

// const MAX_BUFFER_SIZE: usize = 10 * 1024 * 1024; // 10MB

pub fn get_field_name(field: &Field) -> Result<String> {
    let content_disposition = field.content_disposition().clone();
    let field_name = content_disposition.get_name()
        .ok_or(anyhow!(MultipartError::FieldNameNotValid))?;

    Ok(field_name.to_string())
}

pub async fn build_image_file(filename: Option<String>, req: &HttpRequest, upload_dir: String, field: &mut Field) -> Result<ImageFile> {
    let content_length = match req.headers().get(CONTENT_LENGTH) {
        Some(content_length) => content_length.to_str()?.parse::<usize>()?,
        None => return Err(anyhow!(MultipartError::ContentLengthGetFailed)),
    };

    let file_type = field.content_type()
        .ok_or(anyhow!(MultipartError::ContentTypeGetFailed))?
        .clone();

    let file = get_file_buffer(field).await?;
    let filename = filename.unwrap_or("".to_string());

    let image_file = ImageFile::default()
        .set_file(file)
        .set_file_type(file_type)
        .set_file_size(content_length)
        .set_upload_dir(upload_dir)
        .set_filename(filename);

    image_file.validate()?;

    Ok(image_file)
}

async fn get_file_buffer(field: &mut Field) -> Result<Vec<u8>> {
    let mut buffer = vec![];

    while let Ok(Some(chunk)) = field.try_next().await {
        buffer.write_all(&chunk)?
    }

    Ok(buffer)
}