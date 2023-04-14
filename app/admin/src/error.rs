use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use custom_attrs::CustomAttrs;
use serde::Serialize;
use tracing::warn;
use utoipa::ToSchema;

pub type Result<T> = std::result::Result<T, ErrorCode>;

#[derive(Debug, CustomAttrs, ToSchema)]
#[attr(pub status_code: StatusCode)]
#[attr(pub message: Option<&str>)]
pub enum ErrorCode {
    /// 内部服务错误
    #[attr(status_code = StatusCode::INTERNAL_SERVER_ERROR,message = "服务异常")]
    InternalServer(&'static str),
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
    fn from(_value: service::ServiceError) -> Self {
        Self::InternalServer("")
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
                    _ => self.get_message(),
                },
            }),
        )
            .into_response()
    }
}

impl ErrorCode {
    pub fn to_json_string(&self) -> String {
        let resoinse = ErrorResponse {
            code: 1,
            msg: match self {
                Self::ParamsValidator(ref err_str) => Some(err_str),
                _ => self.get_message(),
            },
        };
        serde_json::to_string(&resoinse).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse<'a> {
    #[serde(rename = "errcode")]
    code: i64,
    #[serde(rename = "errmsg")]
    msg: Option<&'a str>,
}
