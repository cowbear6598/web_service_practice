use anyhow::Result;

use web_service_pratice::user::entities::user::User;

use crate::common::fake_data::fake_register_dto;

#[test]
fn test_create_user_from_register() {
    let user = create_user().unwrap();

    let uid_dash_count = user.uid.matches('-').count();

    assert_eq!(4, uid_dash_count);
    assert_eq!("test", user.name);
    assert_eq!("test@gmail.com", user.email);
    assert_ne!("test123456", user.password);
}

#[test]
fn test_verify_password_successful() {
    let user = create_user().unwrap();

    let result = user.verify_password("test123456").unwrap();

    assert_eq!(true, result);
}

#[test]
fn test_verify_password_failed() {
    let user = create_user().unwrap();

    let result = user.verify_password("test1234567").unwrap();

    assert_eq!(false, result);
}

fn create_user() -> Result<User> {
    let register_dto = fake_register_dto();

    let user = User::try_from(register_dto)?;

    Ok(user)
}
