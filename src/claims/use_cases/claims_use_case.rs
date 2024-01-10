use actix_http::header;
use actix_web::HttpRequest;
use anyhow::{anyhow, Result};

use crate::{
    claims::entities::claims::Claims,
    claims::error::claims_error::ClaimsError,
    claims::types::role::Role,
    claims::types::token_type::TokenType,
    claims::use_cases::claims_use_case_trait::ClaimsUseCaseTrait,
};

pub struct ClaimsUseCase {
    secret: String,
}

impl ClaimsUseCaseTrait for ClaimsUseCase {
    fn create_token(&self, sub: String, role: Role, token_type: TokenType, exp: usize) -> Result<String> {
        let exp = exp + chrono::Utc::now().timestamp() as usize;

        let claims = Claims::new(sub, role, token_type, exp);

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        Ok(token)
    }

    fn decode_token(&self, token: String) -> Result<Claims> {
        let mut validation = jsonwebtoken::Validation::default();
        validation.leeway = 0;

        let key = jsonwebtoken::DecodingKey::from_secret(self.secret.as_ref());

        let data = jsonwebtoken::decode::<Claims>(
            token.as_str(),
            &key,
            &validation,
        ).map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => anyhow!(ClaimsError::TokenExpired),
            _ => anyhow!(ClaimsError::InvalidToken),
        })?;

        Ok(data.claims)
    }

    fn decode_token_from_request(&self, req: &HttpRequest) -> Result<Claims> {
        let headers = req.headers();
        let header_value = headers.get(header::AUTHORIZATION)
            .ok_or(anyhow!(ClaimsError::InvalidToken))?;

        let token = header_value.to_str()
            .map_err(|_| anyhow!(ClaimsError::InvalidToken))?
            .to_string();

        let claims = self.decode_token(token)?;

        Ok(claims)
    }
}

impl ClaimsUseCase {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
        }
    }
}

pub fn create_claims_use_case() -> Box<ClaimsUseCase> {
    let secret = std::env::var("SECRET").expect("SECRET 環境變數未設定");

    Box::new(ClaimsUseCase::new(secret))
}