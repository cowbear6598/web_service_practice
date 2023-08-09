use actix_web::HttpRequest;
use anyhow::Result;
use shaku::Interface;
use crate::entities::claims_entity::Claims;

pub trait ClaimsTrait: Interface {
    fn get_claims_from_request(&self, req: &HttpRequest) -> Result<Claims>;
    fn create_token(&self, user_id: String, role: String) -> Result<String>;
    fn create_user_token(&self, user_id: String, role: String) -> Result<(String, String)>;
}