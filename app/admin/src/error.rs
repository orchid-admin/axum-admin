use axum::response::{IntoResponse, Json, Response};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, ErrorCode>;

#[derive(Debug)]
pub enum ErrorCode {
    GenerateToken,
    TokenParse,
    TokenNotExist,
    TokenValid,
    GeneratePassword,
    ServerSteup,
    GenerateCaptcha,
}

impl IntoResponse for ErrorCode {
    fn into_response(self) -> Response {
        Json(ErrorResponse {
            code: 1,
            msg: Some(""),
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
