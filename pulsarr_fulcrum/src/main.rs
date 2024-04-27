use std::env;
use axum::{routing::get, Router};
use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // setup db connection and run any necessary migrations
    let db_url = env!("DATABASE_URL");
    let pool = PgPool::connect(&db_url).await?;

    sqlx::migrate!("db/migrations").run(&pool).await?;

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Welcome to Pulsarr" }));

    // run our app with hyper, listening globally on port 3000 TODO: find a way to make the port environment dependent
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await?;
    axum::serve(listener, app).await?;

    Ok(())
}
