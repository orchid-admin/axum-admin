use crate::{ctls::Claims, error::ErrorCode, state::AppState};
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_auth::AuthBearer;

pub async fn auth<B>(
    AuthBearer(token): AuthBearer,
    State(state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    match state.jwt.lock().await.get_item("admin", &token) {
        Some(jwt_item) => {
            if !jwt_item.check() {
                return Ok(ErrorCode::TokenValid.into_response());
            }

            match state.jwt.lock().await.decode::<Claims>(token) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);
                    Ok(next.run(req).await)
                }
                Err(err) => Ok(err.into_response()),
            }
        }
        None => Ok(ErrorCode::TokenNotExist.into_response()),
    }
}
