pub mod auth;
pub mod dept;
pub mod menu;
pub mod role;
pub mod user;

use crate::{error::ErrorCode, state::AppState};
use axum::{
    body::Body,
    extract::{rejection::MatchedPathRejection, MatchedPath, State},
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, RequestExt,
};
use service::sys_user;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    user_id: i32,
    exp: i128,
}

impl Claims {
    pub fn build(user_id: i32) -> Self {
        Self {
            user_id,
            exp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
        }
    }
}

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

pub async fn access_matched_path(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next<Body>,
) -> Result<Response, StatusCode> {
    let matched_path: Result<MatchedPath, MatchedPathRejection> =
        req.extract_parts::<MatchedPath>().await;
    Ok(match matched_path {
        Ok(path) => {
            match sys_user::check_user_permission(
                &state.db,
                claims.user_id,
                req.method().as_str(),
                path.as_str(),
            )
            .await
            {
                Ok(true) => next.run(req).await,
                _ => ErrorCode::Other("权限不足").into_response(),
            }
        }
        Err(_) => ErrorCode::Other("权限不足").into_response(),
    })
}
