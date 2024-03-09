use super::Claims;
use crate::{
    error::{ErrorCode, Result},
    state::AppState,
};
use axum::{
    body::Body,
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};
use service::{system_menu, system_user};
use utils::{paginate::PaginateParams, password::Password};

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/user", get(index))
        .route("/user/:id", get(info))
        .route("/user", post(create))
        .route("/user/:id", put(update))
        .route("/user/:id", delete(del))
        .route("/user/update_password", post(update_password))
        .route("/user/get_menu", get(get_menu))
        .route("/user/get_user_permission", get(get_user_permission))
        .with_state(state)
}
/// user list
async fn index(
    State(state): State<AppState>,
    Query(params): Query<RequestSearch>,
) -> Result<impl IntoResponse> {
    Ok(Json(system_user::paginate(&state.db, params.into()).await?))
}

/// user`detail
async fn info(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse> {
    Ok(Json(system_user::info(&state.db, id).await?))
}

/// add user
async fn create(
    State(state): State<AppState>,
    Json(param): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    system_user::create(&state.db, param.into()).await?;
    Ok(Body::empty())
}

/// update user by user`id
async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(param): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    system_user::update(&state.db, id, param.into()).await?;
    Ok(Body::empty())
}

/// delete user by user`id
async fn del(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse> {
    system_user::delete(&state.db, id).await?;
    Ok(Body::empty())
}

/// change password
async fn update_password(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(param): Json<RequestUpdatePassword>,
) -> Result<impl IntoResponse> {
    let info = system_user::info(&state.db, claims.user_id).await?;
    if !Password::verify_password(&info.password, &info.salt, param.old_password.as_bytes())? {
        return Err(crate::error::ErrorCode::InputOldPassword);
    }
    if param.new_password.is_empty() {
        return Err(ErrorCode::InputPasswordNotEmpty);
    }
    if param.new_password.eq(&param.confirm_password) {
        return Err(ErrorCode::InputComfirmPasswordDifferentForInputPassword);
    }
    system_user::update_password(&state.db, claims.user_id, &param.new_password).await?;
    Ok(Body::empty())
}

/// get current user menu
async fn get_menu(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    Ok(Json(
        system_menu::get_user_slide_menu_trees(
            &state.db,
            claims.user_id,
            system_menu::Filter {
                menu_types: Some(vec![
                    system_menu::MenuType::Menu,
                    system_menu::MenuType::Redirect,
                    system_menu::MenuType::Iframe,
                    system_menu::MenuType::Link,
                ]),
                ..Default::default()
            },
        )
        .await?,
    ))
}

/// get current user permission
async fn get_user_permission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let info = system_user::get_current_user_info(&state.db, claims.user_id).await?;
    let btn_auths = system_menu::filter_menu_types(
        Some(vec![system_menu::MenuType::BtnAuth]),
        system_menu::get_menu_by_role(&state.db, info.role).await?,
    )
    .into_iter()
    .map(|x| x.btn_auth())
    .collect::<Vec<String>>();

    Ok(Json(UserPermission {
        info: info.user,
        btn_auths,
    }))
}
#[derive(Debug, Deserialize)]
struct RequestSearch {
    keyword: Option<String>,
    role_id: Option<i32>,
    dept_id: Option<i32>,
    status: Option<i32>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<RequestSearch> for system_user::Filter {
    fn from(value: RequestSearch) -> Self {
        Self {
            keyword: value.keyword,
            role_id: value.role_id,
            dept_id: value.dept_id,
            status: value.status,
            paginate: value.paginate,
        }
    }
}

#[derive(Debug, Deserialize)]
struct RequestFormCreate {
    username: String,
    nickname: String,
    role_id: Option<i32>,
    dept_id: Option<i32>,
    phone: String,
    email: String,
    sex: i32,
    password: Option<String>,
    describe: Option<String>,
    expire_time: Option<String>,
    status: i32,
}

impl From<RequestFormCreate> for system_user::FormParamsForCreate {
    fn from(value: RequestFormCreate) -> Self {
        let mut expire_time = None;
        if let Some(expire_time_string) = value.expire_time {
            let description =
                time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
            if let Ok(system_time) = time::OffsetDateTime::parse(&expire_time_string, description)
                .map(|x| std::time::SystemTime::from(x))
            {
                expire_time = Some(system_time);
            }
        }

        let mut salt = String::new();
        let mut password = String::new();
        if let Some(input_password) = value.password {
            let (encode_password, encode_salt) =
                utils::password::Password::generate_hash_salt(input_password.as_bytes()).unwrap();
            salt = encode_salt;
            password = encode_password;
        }

        Self {
            username: value.username,
            nickname: value.nickname,
            role_id: value.role_id,
            dept_id: value.dept_id,
            phone: value.phone,
            email: value.email,
            sex: value.sex,
            password,
            salt,
            describe: value.describe.unwrap_or_default(),
            expire_time,
            status: value.status,
        }
    }
}

#[derive(Debug, Deserialize)]
struct RequestUpdatePassword {
    old_password: String,
    new_password: String,
    confirm_password: String,
}

#[derive(Debug, Serialize)]
struct UserPermission {
    info: system_user::Info,
    btn_auths: Vec<String>,
}
