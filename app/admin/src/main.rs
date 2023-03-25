mod ctls;
mod error;
mod jwt;
mod middleware;
mod router;

#[tokio::main]
async fn main() {
    let app = router::init();
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Hello, world!");
}
