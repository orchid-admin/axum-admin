use crate::{
    error::{ErrorCode, Result},
    state::AppState,
};
use axum::{
    body::Body,
    extract::{self, Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::member;
use utils::paginate::PaginateParams;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/member", get(index))
        .route("/member/:id", get(info))
        .route("/member", post(create))
        .route("/member/:id", put(update))
        .route("/member/:id", delete(del))
        .with_state(state)
}

/// member list
async fn index(
    State(state): State<AppState>,
    Query(params): Query<RequestSearch>,
) -> Result<impl IntoResponse> {
    let data = member::paginate(&state.db, params.into()).await?;
    Ok(Json(data))
}

/// member detail
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(member::info(&state.db, id).await?))
}

/// create member
async fn create(
    State(state): State<AppState>,
    Json(params): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    if member::get_by_email(&state.db, &params.email, None)
        .await?
        .is_some()
    {
        return Err(ErrorCode::EmailExsist);
    }
    member::create(&state.db, &mut params.into()).await?;
    Ok(Body::empty())
}

/// update user
async fn update(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(params): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    if member::get_by_email(&state.db, &params.email, Some(id))
        .await?
        .is_some()
    {
        return Err(ErrorCode::EmailExsist);
    }
    member::update(&state.db, id, params.into()).await?;
    Ok(Body::empty())
}

/// delete member
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    member::info(&state.db, id).await?;
    member::delete(&state.db, id).await?;
    Ok(Body::empty())
}

#[derive(Debug, Deserialize)]
struct RequestSearch {
    keyword: Option<String>,
    sex: Option<i32>,
    status: Option<i32>,
    is_promoter: Option<i32>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<RequestSearch> for member::Filter {
    fn from(value: RequestSearch) -> Self {
        Self {
            keyword: value.keyword,
            sex: value.sex,
            status: value.status,
            is_promoter: value.is_promoter,
            paginate: value.paginate,
        }
    }
}
#[derive(Debug, Deserialize)]
struct RequestFormCreate {
    email: String,
    mobile: Option<String>,
    nickname: Option<String>,
    avatar: Option<String>,
    password: Option<String>,
    #[serde(default)]
    sex: Option<i32>,
    #[serde(default)]
    balance: f64,
    #[serde(default)]
    integral: i32,
    remark: Option<String>,
    #[serde(default)]
    status: i32,
    #[serde(default)]
    is_promoter: i32,
}

impl From<RequestFormCreate> for member::FormParamsForCreate {
    fn from(value: RequestFormCreate) -> Self {
        let mut salt = String::new();
        let mut password = String::new();
        if let Some(input_password) = value.password {
            let (encode_password, encode_salt) =
                utils::password::Password::generate_hash_salt(input_password.as_bytes()).unwrap();
            salt = encode_salt;
            password = encode_password;
        }
        Self {
            email: value.email,
            mobile: value.mobile.unwrap_or_default(),
            nickname: value.nickname.unwrap_or_default(),
            avatar: value.avatar.unwrap_or_default(),
            sex: value.sex.unwrap_or_default(),
            balance: value.balance,
            integral: value.integral,
            remark: value.remark.unwrap_or_default(),
            status: value.status,
            is_promoter: value.is_promoter,
            password,
            salt,
            ..Default::default()
        }
    }
}
