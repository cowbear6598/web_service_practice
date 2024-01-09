use serde::{Deserialize, Serialize};

use crate::{
    claims::types::role::Role,
    claims::types::token_type::TokenType,
};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub token_type: TokenType,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: String, role: Role, token_type: TokenType) -> Self {
        let exp = match token_type {
            TokenType::Access => chrono::Utc::now().timestamp() as usize + 60 * 60 * 24,
            TokenType::Refresh => chrono::Utc::now().timestamp() as usize + 60 * 60 * 24 * 30,
        };

        Self {
            sub,
            role,
            token_type,
            exp,
        }
    }
}