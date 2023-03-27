use error::{ErrorCode, Result};

mod captcha;
mod ctls;
mod error;
mod jwt;
mod middleware;
mod password;
mod router;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let app = router::init();
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .map_err(|_| ErrorCode::ServerSteup)?;
    Ok(())
}
