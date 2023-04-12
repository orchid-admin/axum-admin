use crate::{ctls::Claims, error::ErrorCode, jwt::UseType, state::AppState};
use axum::{
    body::Body,
    extract::{rejection::MatchedPathRejection, MatchedPath, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    RequestExt,
};
use axum_auth::AuthBearer;

pub async fn token_check<B>(
    AuthBearer(token): AuthBearer,
    State(state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let jwt = state.jwt.lock().await;
    match jwt.get_item(&UseType::Admin, &token) {
        Some(jwt_item) => {
            if !jwt_item.check() {
                return Ok(ErrorCode::Unauthorized.into_response());
            }

            match jwt.decode::<Claims>(&token) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);
                    Ok(next.run(req).await)
                }
                Err(err) => Ok(err.into_response()),
            }
        }
        None => Ok(ErrorCode::Unauthorized.into_response()),
    }
}

pub async fn access_matched_path(mut request: Request<Body>) -> Request<Body> {
    let matched_path: Result<MatchedPath, MatchedPathRejection> =
        request.extract_parts::<MatchedPath>().await;

    tracing::info!("{:#?}", matched_path);

    request
}
