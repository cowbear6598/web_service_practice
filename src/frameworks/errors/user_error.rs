use thiserror::Error;
use crate::frameworks::errors::to_u16_trait::ToU16;

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

    #[error("新增用戶失敗")]
    AddUserFail = 2001,
}

impl ToU16 for UserError {
    fn to_u16(self) -> u16 {
        self as u16
    }
}