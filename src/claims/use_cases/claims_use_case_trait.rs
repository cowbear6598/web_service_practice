use actix_web::HttpRequest;
use anyhow::Result;
use mockall::automock;

use crate::{
    claims::entities::claims::Claims,
    claims::types::role::Role,
};

#[automock]
pub trait ClaimsUseCaseTrait: Send + Sync {
    fn create_token(&self, uid: String, role: Role) -> Result<(String, String)>;
    fn decode_token(&self, token: String) -> Result<Claims>;
    fn decode_token_from_request(&self, req: &HttpRequest) -> Result<Claims>;
}