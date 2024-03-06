use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum Types {
    SystemAuthJwt = 0,
    SystemAuthLoginCaptcha = 1,
    SystemAuthLoginMobile = 2,
    SystemAuthLoginQrCode = 3,
    MemberAuthRegisterEmail = 4,
    MemberAuthLoginEmail = 5,
}
impl From<i32> for Types {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::SystemAuthJwt,
            1 => Self::SystemAuthLoginCaptcha,
            2 => Self::SystemAuthLoginMobile,
            3 => Self::MemberAuthRegisterEmail,
            4 => Self::SystemAuthLoginQrCode,
            5 => Self::MemberAuthLoginEmail,
            _ => Self::SystemAuthJwt,
        }
    }
}
impl From<Types> for i32 {
    fn from(value: Types) -> Self {
        match value {
            Types::SystemAuthJwt => 0,
            Types::SystemAuthLoginCaptcha => 1,
            Types::SystemAuthLoginMobile => 2,
            Types::MemberAuthRegisterEmail => 3,
            Types::SystemAuthLoginQrCode => 4,
            Types::MemberAuthLoginEmail => 5,
        }
    }
}
