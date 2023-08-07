use actix_multipart::Field;
use anyhow::{anyhow, Result};
use crate::frameworks::errors::multipart_error::MultipartError;

// const MAX_BUFFER_SIZE: usize = 10 * 1024 * 1024; // 10MB

pub fn get_field_name(field: &Field) -> Result<String> {
    let content_disposition = field.content_disposition().clone();
    let field_name = content_disposition.get_name()
        .ok_or(anyhow!(MultipartError::FieldNameGetFail))?;

    Ok(field_name.to_string())
}