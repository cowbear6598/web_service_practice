pub enum ResponseCode {
    AddUserFail = 1001,
    RemoveUserFail = 1002,
    UploadAvatarFail = 1003,
}

impl ResponseCode {
    pub fn to_u16(self) -> u16 {
        self as u16
    }
}