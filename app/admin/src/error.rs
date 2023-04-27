use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use custom_attrs::CustomAttrs;
use serde::Serialize;
use tracing::warn;

pub type Result<T> = std::result::Result<T, ErrorCode>;

#[derive(Debug, CustomAttrs)]
#[attr(pub status_code: StatusCode)]
#[attr(pub message: Option<&str>)]
pub enum ErrorCode {
    /// 内部服务错误
    #[attr(status_code = StatusCode::INTERNAL_SERVER_ERROR,message = "服务异常")]
    InternalServer(&'static str),
    /// 内部服务错误
    #[attr(status_code = StatusCode::INTERNAL_SERVER_ERROR,message = "服务异常")]
    InternalServerString(String),
    /// TOKEN无效
    #[attr(status_code = StatusCode::UNAUTHORIZED,message = "无效TOKEN")]
    Unauthorized,
    /// 服务启动失败
    #[attr(status_code = StatusCode::INTERNAL_SERVER_ERROR, message = "服务启动失败")]
    ServerSteup,
    /// 请求参数错误
    #[attr(status_code = StatusCode::BAD_REQUEST,message = "请求参数错误")]
    RequestParams(String),
    /// 请求参数验证失败
    #[attr(status_code = StatusCode::BAD_REQUEST)]
    ParamsValidator(String),
    /// 其他错误提示
    #[attr(status_code = StatusCode::BAD_REQUEST)]
    Other(&'static str),
}

impl From<service::ServiceError> for ErrorCode {
    fn from(value: service::ServiceError) -> Self {
        Self::InternalServerString(value.get_code().to_owned())
    }
}

impl From<axum::http::StatusCode> for ErrorCode {
    fn from(value: axum::http::StatusCode) -> Self {
        Self::InternalServerString(value.to_string())
    }
}

impl IntoResponse for ErrorCode {
    fn into_response(self) -> axum::response::Response {
        (
            self.get_status_code(),
            Json(ErrorResponse {
                code: 1,
                msg: match self {
                    Self::InternalServer(err) => {
                        warn!(err);
                        self.get_message()
                    }
                    Self::ParamsValidator(ref err_str) => Some(err_str),
                    Self::RequestParams(ref err_str) => Some(err_str), // todo
                    Self::Other(err_str) => Some(err_str),
                    Self::InternalServerString(ref err_str) => Some(err_str),
                    _ => self.get_message(),
                },
            }),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse<'a> {
    #[serde(rename = "errcode")]
    code: i64,
    #[serde(rename = "errmsg")]
    msg: Option<&'a str>,
}
