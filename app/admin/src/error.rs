use axum::{http::StatusCode, response::IntoResponse};
use custom_attrs::CustomAttrs;

pub type Result<T> = std::result::Result<T, ErrorCode>;

#[derive(Debug, CustomAttrs)]
#[attr(pub status_code: StatusCode)]
#[attr(pub message: Option<&str>)]
pub enum ErrorCode {
    /// 内部服务错误
    #[attr(status_code = StatusCode::INTERNAL_SERVER_ERROR,message = "服务异常")]
    InternalServerString(String),
    /// TOKEN无效
    #[attr(status_code = StatusCode::UNAUTHORIZED,message = "无效TOKEN")]
    Unauthorized,
    /// 服务启动失败
    #[attr(status_code = StatusCode::INTERNAL_SERVER_ERROR, message = "服务启动失败")]
    ServerSteup,
    /// 其他错误提示
    #[attr(status_code = StatusCode::BAD_REQUEST)]
    Other(&'static str),
    #[attr(status_code = StatusCode::BAD_REQUEST)]
    OtherString(String),
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
            service::ServiceError::DataNotFound => "数据不存在".to_owned(),
            service::ServiceError::CacheNotFound => "缓存不存在".to_owned(),
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
        Self::InternalServerString(format!("生成密码错误:{}", msg))
    }
}

impl From<axum::http::StatusCode> for ErrorCode {
    fn from(value: axum::http::StatusCode) -> Self {
        Self::InternalServerString(value.to_string())
    }
}

impl IntoResponse for ErrorCode {
    fn into_response(self) -> axum::response::Response {
        let response = match self {
            Self::Other(err_str) => Some(err_str),
            Self::OtherString(ref err_str) => Some(err_str).map(|x| x.as_str()),
            Self::InternalServerString(ref err_str) => Some(err_str).map(|x| x.as_str()),
            _ => self.get_message(),
        }
        .map(|x| x.to_string());
        (self.get_status_code(), response.unwrap_or_default()).into_response()
    }
}
