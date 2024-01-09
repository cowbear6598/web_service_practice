use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("帳號或密碼錯誤")]
    PasswordNotMatch,
    #[error("找不到使用者")]
    UserNotFound,
}