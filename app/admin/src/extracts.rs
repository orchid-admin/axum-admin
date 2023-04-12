use axum::{
    async_trait,
    extract::{
        rejection::{FormRejection, JsonRejection},
        FromRequest, Json,
    },
    http::Request,
    response::{IntoResponse, Response},
    Form,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

use crate::error::ErrorCode;

#[derive(Debug, Clone, Copy, Default, serde::Deserialize)]
pub struct ValidatorJson<T: Validate>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatorForm<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatorJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = ValidatorError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatorJson(value))
    }
}

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatorForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    B: Send + 'static,
{
    type Rejection = ValidatorError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatorForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ValidatorError {
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ValidatorError {
    fn into_response(self) -> Response {
        match self {
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

                let error = error_array.first().unwrap();
                //format!("[{}]", error).replace('\n', ", ")
                ErrorCode::ParamsValidator(error.clone().unwrap())
            }
            ValidatorError::AxumFormRejection(e) => ErrorCode::RequestParams(e.to_string()),
            ValidatorError::AxumJsonRejection(e) => ErrorCode::RequestParams(e.to_string()),
        }
        .into_response()
    }
}
