use axum::response::{IntoResponse, Json, Response};
use custom_attrs::CustomAttrs;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, ErrorCode>;

#[derive(Debug, CustomAttrs)]
#[attr(pub code: i64)]
#[attr(pub name: &str)]
pub enum ErrorCode {
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
}

impl IntoResponse for ErrorCode {
    fn into_response(self) -> Response {
        Json(ErrorResponse {
            code: self.get_code(),
            msg: Some(self.get_name()),
        })
        .into_response()
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse<'a> {
    #[serde(rename = "errcode")]
    code: i64,
    msg: Option<&'a str>,
}
