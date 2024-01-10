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
    pub fn new(sub: String, role: Role, token_type: TokenType, exp: usize) -> Self {
        Self {
            sub,
            role,
            token_type,
            exp,
        }
    }
}