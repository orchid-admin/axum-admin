mod auth;
mod member;
mod member_bill;
mod member_team;
mod sys_action_log;
mod sys_dept;
mod sys_dict;
mod sys_dict_data;
mod sys_login_log;
mod sys_menu;
mod sys_role;
mod sys_user;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    user_id: i32,
    exp: i128,
}

impl Claims {
    pub fn build(user_id: &i32) -> Self {
        Self {
            user_id: *user_id,
            exp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
        }
    }
}

/// 路由
pub mod router {
    use super::*;
    use crate::state::AppState;
    use axum::{middleware, Router};

    pub async fn init(state: AppState) -> Router {
        Router::new()
            .merge(no_auths(state.clone()))
            .merge(auths(state))
    }

    /// 需授权的路由
    fn auths(state: AppState) -> Router {
        Router::new()
            .merge(sys_user::routers(state.clone()))
            .merge(sys_role::routers(state.clone()))
            .merge(sys_menu::routers(state.clone()))
            .merge(sys_dept::routers(state.clone()))
            .merge(sys_dict::routers(state.clone()))
            .merge(sys_dict_data::routers(state.clone()))
            .merge(sys_login_log::routers(state.clone()))
            .merge(sys_action_log::routers(state.clone()))
            .merge(member::routers(state.clone()))
            .merge(member_team::routers(state.clone()))
            .merge(member_bill::routers(state.clone()))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                middlewares::access_matched_path,
            ))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                middlewares::token_check,
            ))
            .with_state(state)
    }

    fn no_auths(state: AppState) -> Router {
        Router::new().merge(auth::routers(state))
    }
}

/// 中间件
mod middlewares {
    use crate::{error::ErrorCode, state::AppState};
    use axum::{
        async_trait,
        body::Body,
        extract::{
            rejection::MatchedPathRejection, ConnectInfo, FromRequestParts, MatchedPath, State,
        },
        http::{
            header::{AUTHORIZATION, USER_AGENT},
            request::Parts,
            HeaderValue, Request, StatusCode,
        },
        middleware::Next,
        response::{IntoResponse, Response},
        Extension, RequestExt,
    };

    /// 获取请求的User-Agent
    pub struct ExtractUserAgent(pub HeaderValue);
    #[async_trait]
    impl<S> FromRequestParts<S> for ExtractUserAgent
    where
        S: Send + Sync,
    {
        type Rejection = Response;

        async fn from_request_parts(
            parts: &mut Parts,
            _state: &S,
        ) -> Result<Self, Self::Rejection> {
            if let Some(user_agent) = parts.headers.get(USER_AGENT) {
                Ok(ExtractUserAgent(user_agent.clone()))
            } else {
                Err(ErrorCode::Other("请求错误").into_response())
            }
        }
    }

    /// TOKEN检查
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

    /// 解析TOKEN
    async fn parse_token<B>(
        state: AppState,
        req: &Request<B>,
    ) -> crate::error::Result<super::Claims> {
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
        let token_cache_type = service::cache_service::CacheType::SystemAuthJwt;
        let cache = state.cache.lock().await;
        let claims = super::decode_token(token, "secret");
        let jwt_item = cache
            .get(token_cache_type, token, None)
            .await
            .map_err(|_| ErrorCode::Unauthorized)?;
        if !jwt_item.is_valid() {
            return Err(ErrorCode::Unauthorized);
        }

        Ok(claims)
    }

    /// 权限检查
    pub async fn access_matched_path(
        Extension(claims): Extension<super::Claims>,
        ExtractUserAgent(user_agent): ExtractUserAgent,
        ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
        State(state): State<AppState>,
        mut req: Request<Body>,
        next: Next<Body>,
    ) -> Result<Response, StatusCode> {
        let matched_path: Result<MatchedPath, MatchedPathRejection> =
            req.extract_parts::<MatchedPath>().await;
        Ok(match matched_path {
            Ok(path) => {
                match service::system_user_service::check_user_permission(
                    &state.db,
                    claims.user_id,
                    req.method().as_str(),
                    path.as_str(),
                )
                .await
                {
                    Ok(true) => {
                        if let Ok(Some(menu_info)) =
                            service::system_menu_service::get_menu_id_by_api_request(
                                &state.db,
                                req.method().as_str(),
                                path.as_str(),
                            )
                            .await
                        {
                            service::system_action_log_service::create(
                                &state.db,
                                claims.user_id,
                                menu_info.0,
                                &addr.to_string(),
                                service::system_action_log_service::CreateParams {
                                    menu_names: Some(menu_info.1),
                                    ip_address_name: None,
                                    browser_agent: match user_agent.to_str() {
                                        Ok(x) => Some(x.to_owned()),
                                        Err(_) => None,
                                    },
                                },
                            )
                            .await
                            .unwrap();
                        }

                        next.run(req).await
                    }
                    _ => ErrorCode::Other("权限不足").into_response(),
                }
            }
            Err(_) => ErrorCode::Other("权限不足").into_response(),
        })
    }
}

fn decode_token(token: &str, secret: &str) -> Claims {
    jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map(|x| x.claims)
    .unwrap()
}
