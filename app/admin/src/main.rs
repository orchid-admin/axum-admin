use error::{ErrorCode, Result};

mod captcha;
mod ctls;
mod error;
mod jwt;
mod middleware;
mod openapi;
mod password;
mod router;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let captcha = captcha::Captcha::new(2, 10 * 60);
    let jwt = jwt::Jwt::new("secret", 2, 7 * 24 * 60);
    let prisma_client = service::new_client().await?;
    let state = state::State::build(captcha, jwt, prisma_client);

    let app = router::init(state).await;
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .map_err(|_| ErrorCode::ServerSteup)?;
    Ok(())
}
