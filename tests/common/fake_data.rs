use web_service_pratice::user::entities::user::User;
use web_service_pratice::user::types::register_dto::RegisterDto;
use web_service_pratice::user::types::user_response_dto::UserResponseDto;

pub fn fake_register_dto() -> RegisterDto {
    RegisterDto {
        email: "test@gmail.com".to_string(),
        password: "test123456".to_string(),
        name: "test".to_string(),
    }
}

pub fn fake_user_response_dto() -> UserResponseDto {
    UserResponseDto {
        uid: "1".to_string(),
        name: "test".to_string(),
        email: "test@gmail.com".to_string(),
    }
}

pub fn fake_user() -> User {
    User {
        uid: "1".to_string(),
        name: "test".to_string(),
        email: "test@gmail.com".to_string(),
        password: "test123456".to_string(),
    }
}