use sqlx::postgres::PgPool;
use std::env;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World Puslsarr")
    }
}

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
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://0.0.0.0:3003");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/swagger", ui);

    Server::new(TcpListener::bind("0.0.0.0:3003"))
        .run(app)
        .await?;

    Ok(())
}
