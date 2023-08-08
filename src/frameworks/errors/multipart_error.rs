use thiserror::Error;

#[derive(Error, Debug)]
pub enum MultipartError {
    #[error("欄位名稱不允許")]
    FieldNameNotValid,
    #[error("檔案大小取得失敗")]
    ContentLengthGetFailed,
    #[error("檔案類型取得失敗")]
    ContentTypeGetFailed,
    #[error("文字大小超過限制")]
    TextSizeTooBig,
    #[error("欄位缺少資料")]
    FieldMissing,
}