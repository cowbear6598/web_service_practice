use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("認證碼不允許")]
    InvalidToken,
    #[error("認證碼過期")]
    ExpiredToken,
    #[error("認證碼類型不正確")]
    InvalidTokenType,
}