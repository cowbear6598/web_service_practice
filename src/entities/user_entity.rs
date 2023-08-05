use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};
use crate::use_cases::user_use_case::AddUserData;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_role: String,
    pub created_at: String,
    pub last_login_time: String,
}