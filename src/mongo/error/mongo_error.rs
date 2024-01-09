use thiserror::Error;

#[derive(Error, Debug)]
pub enum MongoError {
    #[error("資料庫錯誤: {0}")]
    Exception(String),
}