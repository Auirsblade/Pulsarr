use axum::{routing::get, Router};
use sqlx::postgres::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // setup db connection and run any necessary migrations
    println!("app starting");
    let db_url = env!("DATABASE_URL");
    println!("connecting to db: {db_url}");
    let pool = PgPool::connect(&db_url).await?;
    println!("running migrations");
    sqlx::migrate!("db/migrations").run(&pool).await?;
    println!("migrations complete");

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Welcome to Pulsarr" }));

    // run our app with hyper, listening globally on port 3003
    let listener = tokio::net::TcpListener::bind("localhost:3003").await?;
    println!("serving on 3003");
    axum::serve(listener, app).await?;

    Ok(())
}
