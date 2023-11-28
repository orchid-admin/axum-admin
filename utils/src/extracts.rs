#[derive(Debug, Clone, Copy, Default, serde::Deserialize)]
pub struct ValidatorJson<T: validator::Validate>(pub T);

#[axum::async_trait]
impl<T, S> axum::extract::FromRequest<S> for ValidatorJson<T>
where
    T: serde::de::DeserializeOwned + validator::Validate,
    S: Send + Sync,
    axum::extract::Json<T>:
        axum::extract::FromRequest<S, Rejection = axum::extract::rejection::JsonRejection>,
{
    type Rejection = ValidatorError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let axum::extract::Json(value) = axum::extract::Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatorJson(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatorForm<T>(pub T);

#[axum::async_trait]
impl<T, S> axum::extract::FromRequest<S> for ValidatorForm<T>
where
    T: serde::de::DeserializeOwned + validator::Validate,
    S: Send + Sync,
    axum::Form<T>:
        axum::extract::FromRequest<S, Rejection = axum::extract::rejection::FormRejection>,
{
    type Rejection = ValidatorError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let axum::Form(value) = axum::Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatorForm(value))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ValidatorError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),
    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),
}

impl axum::response::IntoResponse for ValidatorError {
    fn into_response(self) -> axum::response::Response {
        let response = match self {
            ValidatorError::ValidationError(error) => {
                let error_array = error
                    .field_errors()
                    .into_values()
                    .map(|errors| {
                        let error_array = errors
                            .iter()
                            .cloned()
                            .map(|x| match x.message {
                                Some(message) => message.to_string(),
                                None => x.code.to_string(),
                            })
                            .collect::<Vec<String>>();
                        error_array.first().cloned()
                    })
                    .filter(|x| x.is_some())
                    .collect::<Vec<Option<String>>>();

                let error = error_array.first().cloned().unwrap();
                error.unwrap()
            }
            ValidatorError::AxumFormRejection(_) => "请求参数错误".to_string(),
            ValidatorError::AxumJsonRejection(_) => "请求参数错误".to_string(),
        };
        (axum::http::StatusCode::BAD_REQUEST, response).into_response()
    }
}
