pub mod schema;

let mut connection = diesel_async::AsyncPgConnection::establish(&std::env::var("DATABASE_URL")?).await?;