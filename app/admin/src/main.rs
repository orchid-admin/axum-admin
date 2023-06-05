use axum::{extract::MatchedPath, http::Request};
use error::{ErrorCode, Result};
use std::net::SocketAddr;

mod ctls;
mod error;
mod router;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    let env_filter = Some(format!(
        "{}=INFO,tower_http=debug,axum::rejection=trace",
        env!("CARGO_PKG_NAME")
    ));
    utils::logger::init(env_filter);

    let captcha = utils::captcha::Captcha::new(2, 10 * 60);
    let jwt = utils::jwt::Jwt::new("secret", 2, 7 * 24 * 60);
    let prisma_client = service::Database::new(service::DatabaseConfig::default()).await?;
    let state = state::State::build(captcha, jwt, prisma_client);

    let app = router::init(state).await.layer(
        tower_http::trace::TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            tracing::info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
                some_other_field = tracing::field::Empty,
                query = request.uri().query().unwrap_or_default()
            )
        }),
    );
    let server_address = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Service is running on {}", server_address);

    // let (password, salt) = password::Password::generate_hash_salt("123456".as_bytes())?;
    // println!("{}, {}", password, salt);

    axum::Server::bind(&server_address)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .map_err(|_| ErrorCode::ServerSteup)?;
    Ok(())
}
