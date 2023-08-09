use std::env;
use actix_web::HttpRequest;
use crate::{
    adapters::claims_trait::ClaimsTrait,
    entities::claims_entity::Claims,
};
use anyhow::{anyhow, Result};
use jsonwebtoken::{decode, DecodingKey, errors, Validation};
use shaku::Component;
use crate::entities::claims_entity::TokenType;
use crate::frameworks::errors::auth_error::AuthError;

#[derive(Component)]
#[shaku(interface = ClaimsTrait)]
pub struct ClaimsUseCase {
    pub secret: String,
}

impl ClaimsUseCase {
    fn create_claims(&self, user_id: String, role: String, token_type: TokenType) -> Claims {
        let timestamp = chrono::Utc::now().timestamp() as usize;
        let exp = match token_type {
            TokenType::Access => timestamp + 60 * 60 * 24,
            TokenType::Refresh => timestamp + 60 * 60 * 24 * 30,
        };

        Claims {
            sub: user_id,
            exp,
            role,
            token_type,
        }
    }

    fn decode_token(&self, token: &String, token_type: TokenType) -> Result<Claims> {
        let key = DecodingKey::from_secret(self.secret.as_ref());

        let mut validation = Validation::default();
        validation.leeway = 0;

        let token_data = decode::<Claims>(token, &key, &validation)
            .map_err(|err| match err.kind() {
                errors::ErrorKind::ExpiredSignature => anyhow!(AuthError::ExpiredToken),
                _ => anyhow!(AuthError::InvalidToken),
            })?;

        if token_data.claims.token_type != token_type {
            return Err(anyhow!(AuthError::InvalidTokenType));
        }

        Ok(token_data.claims)
    }
}

impl ClaimsTrait for ClaimsUseCase {
    fn get_claims_from_request(&self, req: &HttpRequest) -> Result<Claims> {
        todo!()
    }

    fn create_token(&self, user_id: String, role: String) -> Result<String> {
        todo!()
    }

    fn create_user_token(&self, user_id: String, role: String) -> Result<(String, String)> {
        todo!()
    }
}

impl ClaimsUseCaseParameters {
    pub fn new() -> Self {
        let secret = env::var("TOKEN_SECRET").expect("請設定 TOKEN_SECRET");

        Self {
            secret,
        }
    }
}