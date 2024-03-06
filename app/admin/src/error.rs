use axum::{http::StatusCode, response::IntoResponse};
use custom_attrs::CustomAttrs;

pub type Result<T> = std::result::Result<T, ErrorCode>;

/// Error Code Mapping
#[derive(Debug, CustomAttrs)]
#[attr(pub status_code: StatusCode)]
#[attr(pub message: Option<&str>)]
pub enum ErrorCode {
    /// Internal Server error
    #[attr(status_code = StatusCode::INTERNAL_SERVER_ERROR,message = "Internal Server error")]
    InternalServerString(String),
    /// Token valid
    #[attr(status_code = StatusCode::UNAUTHORIZED,message = "Token valid")]
    Unauthorized,
    /// Server startup error
    #[attr(status_code = StatusCode::INTERNAL_SERVER_ERROR, message = "Server startup error")]
    ServerSteup,
    /// Permissions error
    #[attr(status_code = StatusCode::FORBIDDEN, message = "Permissions error")]
    Permissions,
    /// Get Request User-Agent error
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Get Request User-Agent error")]
    RequestUserAgent,
    /// Captche error
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Captche error")]
    Captche,
    /// Input user`username or user`pwd error
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Input user`username or user`pwd error")]
    InputUserAndPwd,
    /// Not Delete Data
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Not Delete Data")]
    NotDeleteData,
    /// input old Password Error
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Input old Password not empty")]
    InputOldPassword,
    /// input Password not empty Error
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Input Password not empty")]
    InputPasswordNotEmpty,
    /// Input comfirm password is different for input password
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Input comfirm password is different for input password")]
    InputComfirmPasswordDifferentForInputPassword,
    /// Not Change Admin Error
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Not Change Admin")]
    NotChangeAdmin,
    /// Email exsist
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Email exsist")]
    EmailExsist,
    /// Role Sign exsist
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Role Sign exsist")]
    RoleSignExsist,
    /// Dict Sign exsist
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Dict Sign exsist")]
    DictSignExsist,
    /// Dict Data Lable exsist
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "Dict Data Lable exsist")]
    DictDataLableExsist,
    #[attr(status_code = StatusCode::BAD_REQUEST, message = "json error")]
    SerdeJson(serde_json::Error),
}

impl From<service::ServiceError> for ErrorCode {
    fn from(value: service::ServiceError) -> Self {
        let err_string = match value {
            service::ServiceError::BuildClient(err) => format!("BuildDataClient: {}", err),
            service::ServiceError::QueryError(err) => format!("QueryError: {}", err),
            service::ServiceError::RelationNotFetchedError(err) => {
                format!("RelationNotFetchedError: {}", err)
            }
            service::ServiceError::SerializeJson(err) => err.to_string(),
            service::ServiceError::DataNotFound => "DataNotExsist".to_owned(),
            service::ServiceError::CacheNotFound => "CacheNotExsist".to_owned(),
            service::ServiceError::Model(_) => "ModelError".to_owned(),
        };
        Self::InternalServerString(err_string)
    }
}

impl From<utils::password::ErrorType> for ErrorCode {
    fn from(value: utils::password::ErrorType) -> Self {
        let msg = match value {
            utils::password::ErrorType::Argon2(e) => e.to_string(),
            utils::password::ErrorType::Hash(e) => e.to_string(),
        };
        Self::InternalServerString(format!("GeneratePasswordError:{}", msg))
    }
}

impl From<axum::http::StatusCode> for ErrorCode {
    fn from(value: axum::http::StatusCode) -> Self {
        Self::InternalServerString(value.to_string())
    }
}

impl From<serde_json::Error> for ErrorCode {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}
impl IntoResponse for ErrorCode {
    fn into_response(self) -> axum::response::Response {
        let response = match self {
            Self::InternalServerString(ref err_str) => Some(err_str).map(|x| x.as_str()),
            _ => self.get_message(),
        }
        .map(|x| x.to_string());
        (self.get_status_code(), response.unwrap_or_default()).into_response()
    }
}
