use web_service_pratice::user::types::register_dto::RegisterDto;

pub fn fake_register_dto() -> RegisterDto {
    RegisterDto {
        email: "test@gmail.com".to_string(),
        password: "test123456".to_string(),
        name: "test".to_string(),
    }
}