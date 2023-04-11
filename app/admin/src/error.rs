use axum::response::{IntoResponse, Json, Response};
use custom_attrs::CustomAttrs;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, ErrorCode>;

#[derive(Debug, CustomAttrs)]
#[attr(pub code: i64)]
#[attr(pub name: &str)]
pub enum ErrorCode {
    #[attr(code = 10000, name = "Database")]
    Database,
    #[attr(code = 10000, name = "GenerateToken")]
    GenerateToken,
    #[attr(code = 10000, name = "TokenParse")]
    TokenParse,
    #[attr(code = 10000, name = "TokenNotExist")]
    TokenNotExist,
    #[attr(code = 10000, name = "TokenValid")]
    TokenValid,
    #[attr(code = 10000, name = "GeneratePassword")]
    GeneratePassword,
    #[attr(code = 10000, name = "ServerSteup")]
    ServerSteup,
    #[attr(code = 10000, name = "GenerateCaptcha")]
    GenerateCaptcha,
    #[attr(code = 10000, name = "UserNotFound")]
    UserNotFound,
    #[attr(code = 10000, name = "InputPassword")]
    InputPassword,
    #[attr(code = 10000, name = "RequestParams")]
    RequestParams(String),
    #[attr(code = 10000, name = "RequestParamsValidator")]
    RequestParamsValidator(String),
}

impl From<service::ServiceError> for ErrorCode {
    fn from(_value: service::ServiceError) -> Self {
        Self::Database
    }
}

impl IntoResponse for ErrorCode {
    fn into_response(self) -> Response {
        Json(ErrorResponse {
            code: self.get_code(),
            err_code: Some(self.get_name()),
            msg: match self {
                Self::RequestParams(ref err_str) => Some(err_str),
                Self::RequestParamsValidator(ref err_str) => Some(err_str),
                _ => None,
            },
        })
        .into_response()
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse<'a> {
    code: i64,
    err_code: Option<&'a str>,
    msg: Option<&'a str>,
}
