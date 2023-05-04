use crate::{ctls::Claims, error::ErrorCode, state::AppState};
use axum::{
    body::Body,
    extract::{rejection::MatchedPathRejection, MatchedPath, State},
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    RequestExt,
};

/// token检查
pub async fn token_check<B>(
    State(state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    match parse_token(state, &req).await {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(err) => Ok(err.into_response()),
    }
}

async fn parse_token<B>(state: AppState, req: &Request<B>) -> crate::error::Result<Claims> {
    let authorization = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or(ErrorCode::Unauthorized)?
        .to_str()
        .map_err(|_| ErrorCode::Unauthorized)?;

    let (_, token) = authorization
        .split_once(' ')
        .and_then(|(name, token)| {
            if name != "Bearer" {
                return None;
            }
            Some((name, token))
        })
        .ok_or(ErrorCode::Unauthorized)?;
    let jwt = state.jwt.lock().await;
    let claims = jwt.decode::<Claims>(token)?;
    // let jwt_item = jwt
    //     .get_item(&jwt::UseType::Admin, &token)
    //     .ok_or(ErrorCode::Unauthorized)?;
    // if !jwt_item.check() {
    //     return Err(ErrorCode::Unauthorized);
    // }
    Ok(claims)
}

pub async fn access_matched_path(mut request: Request<Body>) -> Request<Body> {
    let matched_path: Result<MatchedPath, MatchedPathRejection> =
        request.extract_parts::<MatchedPath>().await;

    tracing::info!("{:#?}", matched_path);

    request
}
