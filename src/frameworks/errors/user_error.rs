use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("用戶已存在")]
    AlreadyExists = 1001,
    #[error("用戶不存在")]
    NotExists = 1002,
    #[error("用戶帳號或密碼錯誤")]
    PasswordError = 1003,
    #[error("密碼加密失敗")]
    PasswordHashFail = 1004,
}