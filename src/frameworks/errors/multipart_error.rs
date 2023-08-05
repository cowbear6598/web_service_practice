use thiserror::Error;

#[derive(Error, Debug)]
pub enum MultipartError {
    #[error("無法取得欄位名稱")]
    FieldNameGetFail,
}