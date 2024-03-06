use axum::{extract::MatchedPath, http::Request};
use error::{ErrorCode, Result};

/// controllers
mod ctls;
/// error and result
mod error;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    let env_filter = Some(format!(
        "{}=INFO,tower_http=debug,axum::rejection=trace",
        env!("CARGO_PKG_NAME")
    ));
    utils::logger::init(env_filter);

    let config = config::Config::load();
    let db_pool = model::connect::DbConnectPool::new(&config.database_url())?;
    let state = state::State::build(db_pool);

    let app = ctls::router::init(state).await.layer(
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
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("Service is running on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .map_err(|_| ErrorCode::ServerSteup)?;
    Ok(())
}
