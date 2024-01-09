use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FindDto {
    pub email: Option<String>,
}