use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClaimsError {
    #[error("認證無效")]
    InvalidToken,
    #[error("認證已過期")]
    TokenExpired,
}