use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("用戶已存在")]
    AlreadyExists = 1001,
}