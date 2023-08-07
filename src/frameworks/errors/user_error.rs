use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("用戶已存在")]
    AlreadyExists,
    #[error("用戶不存在")]
    NotExists,
    #[error("用戶帳號或密碼錯誤")]
    PasswordError,
    #[error("密碼加密失敗")]
    PasswordHashFail,
    #[error("新增用戶失敗: {0}")]
    AddUserFail(String),
    #[error("移除用戶失敗: {0}")]
    RemoveUserFail(String),
    #[error("上傳頭像失敗: {0}")]
    UploadAvatarFail(String),
}