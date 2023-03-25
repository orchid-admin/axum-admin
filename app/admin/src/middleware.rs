use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_auth::AuthBearer;

use crate::jwt;

pub async fn auth<B>(
    AuthBearer(token): AuthBearer,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    match jwt::Claims::decode(token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(err) => Ok(err.into_response()),
    }
}
